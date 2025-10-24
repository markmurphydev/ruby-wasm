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
  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
]

def get_cells()
  $cells
end

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
    row_lo = neighbor_row == 0 || neighbor_row > 0
    col_lo = neighbor_col == 0 || neighbor_col > 0
    row_hi = neighbor_row < 9 || neighbor_row == 9
    col_hi = neighbor_col < 0 || neighbor_col == 0
    in_bounds = row_lo && col_lo && row_hi && col_hi
    if in_bounds
        alive = $cells[neighbor_row][neighbor_col] == 1
    else
        alive = 0
    end
    if alive
      count = count + 1
    end
  end
  count
end

def step()
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
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
  ]
  for row in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] do
    for col in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] do
      alive = $cells[row][col] == 1
      living_neighbors = count_neighbors(row, col)
      res[row][col] = living_neighbors
    end
  end
  $cells = res
end
