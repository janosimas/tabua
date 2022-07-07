use crate::board::grid::{Grid, GridBuilder, GridExt, Position2d};

pub trait BreadthFirstSearch {
    fn bfs(&self, root: &Position2d, goal: &Position2d) -> Option<Vec<Position2d>>;
}

impl<T> BreadthFirstSearch for Grid<T> {
    fn bfs(&self, root: &Position2d, goal: &Position2d) -> Option<Vec<Position2d>> {
        let mut explored: Grid<bool> = GridBuilder::new_square_grid()
            .with_rows(self.row_len())
            .with_columns(self.column_len())
            .with_initial_value(false)
            .build();

        *explored.get_mut(root).unwrap() = true;
        let mut v = vec![vec![root.clone()]];
        while let Some(path) = v.pop() {
            let p = path.last().unwrap();
            if *p == *goal {
                return Some(path);
            }

            for (next, _) in self.adjacent_cells(&p) {
                if let Some(false) = explored.get(&next) {
                    *explored.get_mut(&next).unwrap() = true;

                    let mut new_path = path.clone();
                    new_path.push(next);
                    v.insert(0, new_path);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::board::grid::CellAdjacency;

    use super::*;

    #[test]
    fn adjacent_goal() {
        let grid =
            Grid::new_square_grid(vec![vec![(), (), ()], vec![(), (), ()], vec![(), (), ()]]);

        let root = (0, 0);
        let goal = (0, 1);

        assert_eq!(grid.bfs(&root, &goal), Some(vec![root, goal]))
    }

    #[test]
    fn line_goal() {
        let grid =
            Grid::new_square_grid(vec![vec![(), (), ()], vec![(), (), ()], vec![(), (), ()]]);

        let root = (0, 0);
        let goal = (0, 2);

        assert_eq!(grid.bfs(&root, &goal), Some(vec![root, (0, 1), goal]))
    }

    #[test]
    fn diagonal_goal() {
        let grid =
            Grid::new_square_grid(vec![vec![(), (), ()], vec![(), (), ()], vec![(), (), ()]]);

        let root = (0, 0);
        let goal = (2, 2);

        assert_eq!(
            grid.bfs(&root, &goal),
            Some(vec![root, (0, 1), (0, 2), (1, 2), goal])
        )
    }

    #[test]
    fn diagonal_goal_2() {
        let grid = GridBuilder::new_square_grid()
            .with_cell_adjacency(CellAdjacency::SideAndVertex)
            .with_rows(3)
            .with_columns(3)
            .with_initial_value(())
            .build();

        let root = (0, 0);
        let goal = (2, 2);

        assert_eq!(grid.bfs(&root, &goal), Some(vec![root, (1, 1), goal]))
    }

    #[test]
    fn l_goal() {
        let grid = GridBuilder::new_square_grid()
            .with_rows(3)
            .with_columns(3)
            .with_initial_value(())
            .build();

        let root = (0, 0);
        let goal = (1, 2);

        assert_eq!(
            grid.bfs(&root, &goal),
            Some(vec![root, (0, 1), (0, 2), goal])
        )
    }

    #[test]
    fn l_goal_2() {
        let grid = GridBuilder::new_square_grid()
            .with_cell_adjacency(CellAdjacency::SideAndVertex)
            .with_rows(3)
            .with_columns(3)
            .with_initial_value(())
            .build();

        let root = (0, 0);
        let goal = (1, 2);

        assert_eq!(grid.bfs(&root, &goal), Some(vec![root, (0, 1), goal]))
    }
}
