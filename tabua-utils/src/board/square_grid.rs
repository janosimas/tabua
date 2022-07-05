use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

pub type RowIterator<'a, T> = std::slice::Iter<'a, Vec<T>>;
pub type RowCellIterator<'a, T> = std::slice::Iter<'a, T>;
/// Helper trait for usefull square grid board functions
pub trait SquareGridExt<'a, T> {
    fn iter_row(&self, row: usize) -> RowCellIterator<T>;
    fn iter_column(&self, col: usize) -> ColumnCellIterator<T>;

    fn row_len(&self) -> usize;
    fn column_len(&self) -> usize;

    fn rows(&self) -> RowIterator<T>;
    fn columns(&self) -> ColumnIterator<T>;

    fn get(&'a self, row: usize, col: usize) -> Option<&'a T>;
    fn get_mut(&'a mut self, row: usize, col: usize) -> Option<&'a mut T>;
}

#[derive(Debug)]
pub struct SquareGridBuilder<T: Clone> {
    rows: Option<usize>,
    columns: Option<usize>,
    initial_value: Option<T>,
}

impl<T: Clone> SquareGridBuilder<T> {
    pub fn new() -> Self {
        Self {
            rows: None,
            columns: None,
            initial_value: None,
        }
    }

    pub fn with_rows(&mut self, rows: usize) -> &mut Self {
        self.rows = Some(rows);
        self
    }

    pub fn with_columns(&mut self, columns: usize) -> &mut Self {
        self.columns = Some(columns);
        self
    }

    pub fn with_initial_value(&mut self, initial_value: T) -> &mut Self {
        self.initial_value = Some(initial_value);
        self
    }

    pub fn build(&self) -> SquareGrid<T> {
        let rows = self.rows.unwrap();
        let columns = self.columns.unwrap();
        let initial_value = self.initial_value.clone().unwrap();

        let grid = std::iter::repeat_with(|| {
            std::iter::repeat_with(|| initial_value.clone())
                .take(columns)
                .collect()
        })
        .take(rows)
        .collect();

        SquareGrid { grid }
    }
}

impl<T: Clone> Default for SquareGridBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SquareGrid<T> {
    grid: Vec<Vec<T>>,
}

impl<T: Clone> SquareGrid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        Self { grid }
    }

    pub fn builder() -> SquareGridBuilder<T> {
        SquareGridBuilder::default()
    }
}

impl<'a, T> SquareGridExt<'a, T> for SquareGrid<T> {
    fn iter_row(&self, row: usize) -> RowCellIterator<T> {
        self.grid.iter_row(row)
    }
    fn iter_column(&self, col: usize) -> ColumnCellIterator<T> {
        self.grid.iter_column(col)
    }

    fn row_len(&self) -> usize {
        self.grid.row_len()
    }
    fn column_len(&self) -> usize {
        self.grid.column_len()
    }

    fn rows(&self) -> RowIterator<T> {
        self.grid.rows()
    }
    fn columns(&self) -> ColumnIterator<T> {
        self.grid.columns()
    }

    fn get(&'a self, row: usize, col: usize) -> Option<&'a T> {
        self.grid.get(row, col)
    }
    fn get_mut(&'a mut self, row: usize, col: usize) -> Option<&'a mut T> {
        self.grid.get_mut(row, col)
    }
}

impl<'a, T> SquareGridExt<'a, T> for Vec<Vec<T>> {
    fn iter_row(&self, row: usize) -> RowCellIterator<T> {
        self[row].iter()
    }
    fn iter_column(&self, col: usize) -> ColumnCellIterator<T> {
        ColumnCellIterator::new(self, col)
    }

    fn row_len(&self) -> usize {
        self.len()
    }
    fn column_len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        self[0].len()
    }

    fn rows(&self) -> RowIterator<T> {
        self.iter()
    }
    fn columns(&self) -> ColumnIterator<T> {
        ColumnIterator::new(self)
    }

    fn get(&'a self, row: usize, col: usize) -> Option<&'a T> {
        self.deref().get(row)?.get(col)
    }
    fn get_mut(&'a mut self, row: usize, col: usize) -> Option<&'a mut T> {
        self.deref_mut().get_mut(row)?.get_mut(col)
    }
}

pub struct ColumnIterator<'a, T: 'a> {
    board: &'a dyn SquareGridExt<'a, T>,
    row: usize,
}

impl<'a, T> ColumnIterator<'a, T> {
    pub fn new(board: &'a dyn SquareGridExt<'a, T>) -> Self {
        Self { board, row: 0 }
    }
}

impl<'a, T> Iterator for ColumnIterator<'a, T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = Vec::with_capacity(self.board.column_len());
        for col in 0..self.board.column_len() {
            result.push(self.board.get(self.row, col).expect("inside column range"));
        }

        self.row += 1;
        Some(result)
    }
}

pub struct ColumnCellIterator<'a, T> {
    board: &'a dyn SquareGridExt<'a, T>,
    col: usize,
    row: usize,
}

impl<'a, T> ColumnCellIterator<'a, T> {
    pub fn new(board: &'a dyn SquareGridExt<'a, T>, col: usize) -> Self {
        Self { board, col, row: 0 }
    }
}

impl<'a, T> Iterator for ColumnCellIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.board.get(self.row, self.col);
        self.row += 1;
        result
    }
}
