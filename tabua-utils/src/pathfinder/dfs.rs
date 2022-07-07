use crate::board::grid::{Grid, GridBuilder, GridExt, Position2d};

pub trait BreadthFirstSearch {
    fn dfs(&self, root: &Position2d, goal: &Position2d) -> Option<Vec<Position2d>>;
}

impl<T> BreadthFirstSearch for Grid<T> {
    fn dfs(&self, root: &Position2d, goal: &Position2d) -> Option<Vec<Position2d>> {
        let mut explored: Grid<bool> = GridBuilder::new_square_grid()
            .with_rows(self.row_len())
            .with_columns(self.column_len())
            .with_initial_value(false)
            .build();

        let mut stack = vec![(*root, vec![*root])];
        while let Some((current, path)) = stack.pop() {
            if let Some(false) = explored.get(&current) {
                if current == *goal {
                    return Some(path);
                }
                *explored.get_mut(&current).unwrap() = true;
                for (next, _) in self.adjacent_cells(&current) {
                    if let Some(false) = explored.get(&next) {
                        let mut new_path = path.clone();
                        new_path.push(next);
                        stack.push((next, new_path));
                    }
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

        assert!(grid.dfs(&root, &goal).is_some())
    }

    #[test]
    fn line_goal() {
        let grid =
            Grid::new_square_grid(vec![vec![(), (), ()], vec![(), (), ()], vec![(), (), ()]]);

        let root = (0, 0);
        let goal = (0, 2);

        assert!(grid.dfs(&root, &goal).is_some())
    }

    #[test]
    fn diagonal_goal() {
        let grid =
            Grid::new_square_grid(vec![vec![(), (), ()], vec![(), (), ()], vec![(), (), ()]]);

        let root = (0, 0);
        let goal = (2, 2);

        assert_eq!(
            grid.dfs(&root, &goal),
            Some(vec![root, (1, 0), (2, 0), (2, 1), goal])
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

        assert_eq!(grid.dfs(&root, &goal), Some(vec![root, (1, 1), goal]))
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

        assert!(grid.dfs(&root, &goal).is_some())
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

        assert!(grid.dfs(&root, &goal).is_some())
    }
}
