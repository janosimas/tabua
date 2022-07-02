pub trait Board2D<'a, T> {
    fn iter_row(&self, row: usize) -> std::slice::Iter<T>;
    fn iter_column(&self, col: usize) -> Board2DColumnIterator<T>;

    fn get(&'a self, row: usize, col: usize) -> &'a T;
    fn get_mut(&'a mut self, row: usize, col: usize) -> &'a mut T;
}

impl<'a, T> Board2D<'a, T> for Vec<Vec<T>> {
    fn iter_row(&self, row: usize) -> std::slice::Iter<T> {
        self[row].iter()
    }

    fn iter_column(&self, col: usize) -> Board2DColumnIterator<T> {
        Board2DColumnIterator {
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

pub struct Board2DColumnIterator<'a, T> {
    board: &'a dyn Board2D<'a, T>,
    col: usize,
    row: usize,
}

impl<'a, T> Iterator for Board2DColumnIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.board.get(self.row, self.col);
        self.row += 1;
        Some(result)
    }
}
