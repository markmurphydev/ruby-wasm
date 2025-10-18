
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

def set_cell(row, col, val)
  $cells[row][col] = val
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


set_cell(4, 5, 1)
set_cell(5, 6, 1)
set_cell(6, 4, 1)
set_cell(6, 5, 1)
set_cell(6, 6, 1)

print_cells

10.times do
  step
  print_cells
end
