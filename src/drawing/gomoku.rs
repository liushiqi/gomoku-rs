use std::ops::Not;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GomokuGrid {
    White,
    Black,
}

impl Not for GomokuGrid {
    type Output = GomokuGrid;

    fn not(self) -> Self::Output {
        match self {
            GomokuGrid::White => GomokuGrid::Black,
            GomokuGrid::Black => GomokuGrid::White,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GomokuData {
    pub size: u32,
    pub chess: Vec<(u32, u32, GomokuGrid)>,
    pub current: GomokuGrid,
}

impl Default for GomokuData {
    fn default() -> Self {
        Self {
            size: 19,
            chess: Vec::new(),
            current: GomokuGrid::Black,
        }
    }
}

impl GomokuData {
    pub fn undo(&mut self) {
        self.chess.pop();
        self.current = !self.current;
    }

    pub fn place(&mut self, x: u32, y: u32) {
        if let None = self.check_grid(x, y) {
            self.chess.push((x, y, self.current));
            self.current = !self.current;
        }
    }

    pub fn check_grid(&self, x: u32, y: u32) -> Option<GomokuGrid> {
        for (gx, gy, grid) in &self.chess {
            if *gx == x && *gy == y {
                return Some(*grid);
            }
        }
        None
    }

    pub fn has_winner(&self) -> Option<GomokuGrid> {
        let last_chess = self.chess.last().unwrap();
        for (dx, dy) in &[(1, 0), (0, 1), (1, 1), (1, -1)] {
            let mut count = 1;
            for i in 1..5 {
                let x = last_chess.0 as i32 + dx * i;
                let y = last_chess.1 as i32 + dy * i;
                if x < 0 || y < 0 || x >= self.size as i32 || y >= self.size as i32 {
                    break;
                }
                if let Some(grid) = self.check_grid(x as u32, y as u32) {
                    if grid == last_chess.2 {
                        count += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            for i in 1..5 {
                let x = last_chess.0 as i32 - dx * i;
                let y = last_chess.1 as i32 - dy * i;
                if x < 0 || y < 0 || x >= self.size as i32 || y >= self.size as i32 {
                    break;
                }
                if let Some(grid) = self.check_grid(x as u32, y as u32) {
                    if grid == last_chess.2 {
                        count += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if count >= 5 {
                return Some(last_chess.2);
            }
        };
        None
    }
}

impl<'a> IntoIterator for &'a GomokuData {
    type Item = &'a (u32, u32, GomokuGrid);
    type IntoIter = std::slice::Iter<'a, (u32, u32, GomokuGrid)>;

    fn into_iter(self) -> Self::IntoIter {
        self.chess.iter()
    }
}

impl GomokuData {
    pub fn iter(&self) -> std::slice::Iter<'_, (u32, u32, GomokuGrid)> {
        self.into_iter()
    }
}
