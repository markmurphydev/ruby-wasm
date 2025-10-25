def range(length)
  res = []
  col = 0
  while col < length
    res = res.push(col)
    col = col + 1
  end
  res
end

def line(length)
  res = []
  col = 0
  while col < length
    res = res.push(0)
    col = col + 1
  end
  res
end

def grid()
  grid = []
  row = 0
  while row < $length
    grid = grid.push(line($length))
    row = row + 1
  end
  grid
end

def update_length(length)
  $length = length
  $cells = grid()
end

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
  res = []
  for row in range($length) do
    res_row = []
    for col in range($length) do
      alive = $cells[row][col] == 1
      living_neighbors = count_neighbors(row, col)

      ln3 = living_neighbors == 3
      alive_ln2 = alive && living_neighbors == 2
      res_alive = ln3 || alive_ln2
      if res_alive
        res_row = res_row.push(1)
      else
        res_row = res_row.push(0)
      end
    end
    res = res.push(res_row)
  end
  $cells = res
end

$length = 15
$cells = grid()
