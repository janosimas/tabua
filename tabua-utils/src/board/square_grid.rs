use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

pub type RowIterator<'a, T> = std::slice::Iter<'a, Vec<T>>;
pub type RowCellIterator<'a, T> = std::slice::Iter<'a, T>;

pub type Position2d = (usize, usize);
pub type Position2dDiff = (i32, i32);

trait AddTuple<T> {
    type Output;

    fn add(self, rhs: T) -> Self::Output;
}

impl AddTuple<Position2dDiff> for Position2d {
    type Output = Option<Position2d>;

    fn add(self, rhs: Position2dDiff) -> Self::Output {
        Some((
            self.0.checked_add_signed(rhs.0.try_into().ok()?)?,
            self.1.checked_add_signed(rhs.1.try_into().ok()?)?,
        ))
    }
}

/// Helper trait for useful square grid board functions
pub trait GridExt<'a, T> {
    fn iter_row(&self, row: usize) -> RowCellIterator<T>;
    fn iter_column(&self, col: usize) -> ColumnCellIterator<T>;

    fn row_len(&self) -> usize;
    fn column_len(&self) -> usize;

    fn rows(&self) -> RowIterator<T>;
    fn columns(&self) -> ColumnIterator<T>;

    fn get(&'a self, position: &Position2d) -> Option<&'a T>;
    fn get_mut(&'a mut self, position: &Position2d) -> Option<&'a mut T>;

    fn adjacencies(&self) -> Vec<Position2dDiff>;
    fn adjacent_cells(&'a self, current: &Position2d) -> Vec<(Position2d, &'a T)>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellType {
    Square,
    Hex,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellAdjacency {
    Side,
    Vertex,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
    cell_type: CellType,
    cell_adjacency: CellAdjacency,
}

impl<T: Clone> Grid<T> {
    pub fn new_square_grid(grid: Vec<Vec<T>>) -> Self {
        Self {
            grid,
            cell_type: CellType::Square,
            cell_adjacency: CellAdjacency::Side,
        }
    }

    pub fn new_hex_grid(grid: Vec<Vec<T>>) -> Self {
        Self {
            grid,
            cell_type: CellType::Hex,
            cell_adjacency: CellAdjacency::Side,
        }
    }

    pub fn builder() -> GridBuilder<T> {
        GridBuilder::new()
    }
}

impl<'a, T> GridExt<'a, T> for Grid<T> {
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

    fn get(&'a self, pos: &Position2d) -> Option<&'a T> {
        self.grid.get(pos)
    }
    fn get_mut(&'a mut self, pos: &Position2d) -> Option<&'a mut T> {
        self.grid.get_mut(pos)
    }

    fn adjacencies(&self) -> Vec<Position2dDiff> {
        match (&self.cell_type, &self.cell_adjacency) {
            #[rustfmt::skip]
            (CellType::Square, CellAdjacency::Side) => vec![
                        (-1, 0),
                (0, -1),         (0, 1),
                        ( 1, 0)],
            #[rustfmt::skip]
            (CellType::Square, CellAdjacency::Vertex) => vec![
                (-1, -1), (-1, 0), (-1, 1),
                ( 0, -1),          ( 0, 1),
                ( 1, -1), ( 1, 0), ( 1, 1),
            ],
            (CellType::Hex, CellAdjacency::Side) => todo!(),
            (CellType::Hex, CellAdjacency::Vertex) => todo!(),
        }
    }

    fn adjacent_cells(&'a self, current: &Position2d) -> Vec<(Position2d, &'a T)> {
        self.grid.adjacent_cells(current)
    }
}

impl<'a, T> GridExt<'a, T> for Vec<Vec<T>> {
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

    fn get(&'a self, pos: &Position2d) -> Option<&'a T> {
        self.deref().get(pos.0)?.get(pos.1)
    }
    fn get_mut(&'a mut self, pos: &Position2d) -> Option<&'a mut T> {
        self.deref_mut().get_mut(pos.0)?.get_mut(pos.1)
    }

    fn adjacencies(&self) -> Vec<Position2dDiff> {
        vec![(0, -1), (0, 1), (-1, 0), (1, 0)]
    }

    fn adjacent_cells(&'a self, current: &Position2d) -> Vec<(Position2d, &'a T)> {
        self.adjacencies()
            .into_iter()
            .filter_map(|diff| current.add(diff))
            .filter_map(|p| Some((p, self.get(&p)?)))
            .collect()
    }
}

#[derive(Debug)]
pub struct GridBuilder<T: Clone> {
    rows: Option<usize>,
    columns: Option<usize>,
    initial_value: Option<T>,
    cell_type: Option<CellType>,
    cell_adjacency: Option<CellAdjacency>,
}

impl<T: Clone> GridBuilder<T> {
    pub fn new() -> Self {
        Self {
            rows: None,
            columns: None,
            initial_value: None,
            cell_type: None,
            cell_adjacency: None,
        }
    }

    pub fn new_square_grid() -> Self {
        Self {
            rows: None,
            columns: None,
            initial_value: None,
            cell_type: Some(CellType::Square),
            cell_adjacency: Some(CellAdjacency::Side),
        }
    }

    pub fn new_hey_grid() -> Self {
        Self {
            rows: None,
            columns: None,
            initial_value: None,
            cell_type: Some(CellType::Hex),
            cell_adjacency: Some(CellAdjacency::Side),
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

    pub fn with_cell_type(&mut self, cell_type: CellType) -> &mut Self {
        self.cell_type = Some(cell_type);
        self
    }

    pub fn with_cell_adjacency(&mut self, cell_adjacency: CellAdjacency) -> &mut Self {
        self.cell_adjacency = Some(cell_adjacency);
        self
    }

    pub fn build(&self) -> Grid<T> {
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

        Grid {
            grid,
            cell_type: self.cell_type.unwrap(),
            cell_adjacency: self.cell_adjacency.unwrap(),
        }
    }
}

impl<T: Clone> Default for GridBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ColumnIterator<'a, T: 'a> {
    board: &'a dyn GridExt<'a, T>,
    row: usize,
}

impl<'a, T> ColumnIterator<'a, T> {
    pub fn new(board: &'a dyn GridExt<'a, T>) -> Self {
        Self { board, row: 0 }
    }
}

impl<'a, T> Iterator for ColumnIterator<'a, T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = Vec::with_capacity(self.board.column_len());
        for col in 0..self.board.column_len() {
            result.push(
                self.board
                    .get(&(self.row, col))
                    .expect("inside column range"),
            );
        }

        self.row += 1;
        Some(result)
    }
}

pub struct ColumnCellIterator<'a, T> {
    board: &'a dyn GridExt<'a, T>,
    col: usize,
    row: usize,
}

impl<'a, T> ColumnCellIterator<'a, T> {
    pub fn new(board: &'a dyn GridExt<'a, T>, col: usize) -> Self {
        Self { board, col, row: 0 }
    }
}

impl<'a, T> Iterator for ColumnCellIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.board.get(&(self.row, self.col));
        self.row += 1;
        result
    }
}
