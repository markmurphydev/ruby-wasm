require "js"

$cells = [
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
]

def get_cells; $cells end

def toggle_cell(row, col)
  if $cells[row][col] == 1
    res = 0
  else
    res = 1
  end
  $cells[row][col] = res
end

def count_neighbors(row, col)
  count = 0
  for neighbor in [
    [row-1, col-1], [row-1, col], [row-1, col+1],
    [row, col-1], [row, col+1],
    [row+1, col-1], [row+1, col], [row+1, col+1]
  ] do
    neighbor_row = neighbor[0]
    neighbor_col = neighbor[1]
    in_bounds = !(neighbor_row < 0 || neighbor_col < 0 || neighbor_row > 9 || neighbor_col > 9)
    alive = in_bounds && $cells[neighbor_row][neighbor_col] == 1
    if alive
      count = count + 1
    end
  end
  count
end

def step
  res = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  ]
  for row in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] do
    for col in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] do
      alive = $cells[row][col] == 1
      living_neighbors = count_neighbors(row, col)
      res_alive = living_neighbors == 3 || (alive && living_neighbors == 2)
      if res_alive
        res[row][col] = 1
      else
        res[row][col] = 0
      end
    end
  end
  $cells = res
end

def print_cells
  for row in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] do
    for col in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] do
      if $cells[row][col] == 1
        print("X ")
      else
        print("_ ")
      end
    end
    print("\n")
  end
  print("\n")
end

def print_neighbors
  res = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  ]
  (0..9).each do |row|
    (0..9).each do |col|
      res[row][col] = count_neighbors(row, col)
    end
  end

  puts res.map { |x| x.join(' ') }
end


toggle_cell(4, 5)
toggle_cell(5, 6)
toggle_cell(6, 4)
toggle_cell(6, 5)
toggle_cell(6, 6)

def js_get_cells
  $cells.to_js
end

def js_toggle_cell(row, col)
  toggle_cell(row.to_i, col.to_i)
end

JS.global[:getCells] = lambda {
  js_get_cells
}
JS.global[:toggleCell] = lambda { |row, col|
  js_toggle_cell(row, col)
}
JS.global[:step] = lambda { step }

if JS.global[:onRuby]
  JS.global[:onRuby].call(:call)
end