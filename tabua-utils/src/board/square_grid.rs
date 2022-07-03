pub trait SquareGridExt<'a, T> {
    fn iter_row(&self, row: usize) -> std::slice::Iter<T>;
    fn iter_column(&self, col: usize) -> SquareGridColumnIterator<T>;

    fn get(&'a self, row: usize, col: usize) -> &'a T;
    fn get_mut(&'a mut self, row: usize, col: usize) -> &'a mut T;
}

pub struct SquareGrid<T, const ROWS: usize, const COLUMNS: usize> {
    grid: [[T; COLUMNS]; ROWS],
}

impl<T, const ROWS: usize, const COLUMNS: usize> SquareGrid<T, ROWS, COLUMNS> {
    pub fn new(grid: [[T; COLUMNS]; ROWS]) -> Self { Self { grid } }
}

impl<'a, T, const ROWS: usize, const COLUMNS: usize> SquareGridExt<'a, T>
    for SquareGrid<T, ROWS, COLUMNS>
{
    fn iter_row(&self, row: usize) -> std::slice::Iter<T> {
        self.grid[row].iter()
    }

    fn iter_column(&self, col: usize) -> SquareGridColumnIterator<T> {
        SquareGridColumnIterator {
            board: self,
            col,
            row: 0,
        }
    }

    fn get(&'a self, row: usize, col: usize) -> &'a T {
        &self.grid[row][col]
    }

    fn get_mut(&'a mut self, row: usize, col: usize) -> &'a mut T {
        &mut self.grid[row][col]
    }
}

impl<'a, T> SquareGridExt<'a, T> for Vec<Vec<T>> {
    fn iter_row(&self, row: usize) -> std::slice::Iter<T> {
        self[row].iter()
    }

    fn iter_column(&self, col: usize) -> SquareGridColumnIterator<T> {
        SquareGridColumnIterator {
            board: self,
            col,
            row: 0,
        }
    }

    fn get(&'a self, row: usize, col: usize) -> &'a T {
        &self[row][col]
    }

    fn get_mut(&'a mut self, row: usize, col: usize) -> &'a mut T {
        &mut self[row][col]
    }
}


impl<'a, T, const ROWS: usize, const COLUMNS: usize> SquareGridExt<'a, T> for [[T; ROWS]; COLUMNS] {
    fn iter_row(&self, row: usize) -> std::slice::Iter<T> {
        self[row].iter()
    }

    fn iter_column(&self, col: usize) -> SquareGridColumnIterator<T> {
        SquareGridColumnIterator {
            board: self,
            col,
            row: 0,
        }
    }

    fn get(&'a self, row: usize, col: usize) -> &'a T {
        &self[row][col]
    }

    fn get_mut(&'a mut self, row: usize, col: usize) -> &'a mut T {
        &mut self[row][col]
    }
}

pub struct SquareGridColumnIterator<'a, T> {
    board: &'a dyn SquareGridExt<'a, T>,
    col: usize,
    row: usize,
}

impl<'a, T> Iterator for SquareGridColumnIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.board.get(self.row, self.col);
        self.row += 1;
        Some(result)
    }
}
