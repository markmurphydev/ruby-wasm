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
    col_hi = neighbor_col < 9 || neighbor_col == 9
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
      ln3 = living_neighbors == 3
      alive_ln2 = alive && living_neighbors == 2
      res_alive = ln3 || alive_ln2
      if res_alive
        res[row][col] = 1
      else
        res[row][col] = 0
      end
    end
  end
  $cells = res
end
