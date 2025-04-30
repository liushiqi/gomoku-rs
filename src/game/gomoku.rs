use std::ops::Not;

use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GomokuColor {
    #[default]
    Black,
    White,
}

impl Display for GomokuColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GomokuColor::White => write!(f, "White"),
            GomokuColor::Black => write!(f, "Black"),
        }
    }
}

impl Not for GomokuColor {
    type Output = GomokuColor;

    fn not(self) -> Self::Output {
        match self {
            GomokuColor::White => GomokuColor::Black,
            GomokuColor::Black => GomokuColor::White,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GomokuChess {
    pub x: u32,
    pub y: u32,
    pub color: GomokuColor,
}

impl GomokuChess {
    pub fn new(x: u32, y: u32, color: GomokuColor) -> Self {
        Self { x, y, color }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GomokuData {
    pub size: u32,
    pub chess: Vec<GomokuChess>,
    pub current: GomokuColor,
}

impl Default for GomokuData {
    fn default() -> Self {
        Self {
            size: 19,
            chess: Vec::new(),
            current: GomokuColor::Black,
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
            self.chess.push(GomokuChess::new(x, y, self.current));
            self.current = !self.current;
        }
    }

    pub fn check_grid(&self, x: u32, y: u32) -> Option<GomokuColor> {
        for GomokuChess {
            x: gx,
            y: gy,
            color,
        } in &self.chess
        {
            if *gx == x && *gy == y {
                return Some(*color);
            }
        }
        None
    }

    pub fn has_winner(&self) -> Option<GomokuColor> {
        let last_chess = self.chess.last().unwrap();
        for (dx, dy) in &[(1, 0), (0, 1), (1, 1), (1, -1)] {
            let mut count = 1;
            for i in 1..5 {
                let x = last_chess.x as i32 + dx * i;
                let y = last_chess.y as i32 + dy * i;
                if x < 0 || y < 0 || x >= self.size as i32 || y >= self.size as i32 {
                    break;
                }
                if let Some(grid) = self.check_grid(x as u32, y as u32) {
                    if grid == last_chess.color {
                        count += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            for i in 1..5 {
                let x = last_chess.x as i32 - dx * i;
                let y = last_chess.y as i32 - dy * i;
                if x < 0 || y < 0 || x >= self.size as i32 || y >= self.size as i32 {
                    break;
                }
                if let Some(grid) = self.check_grid(x as u32, y as u32) {
                    if grid == last_chess.color {
                        count += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            if count >= 5 {
                return Some(last_chess.color);
            }
        }
        None
    }
}

impl<'a> IntoIterator for &'a GomokuData {
    type Item = &'a GomokuChess;
    type IntoIter = std::slice::Iter<'a, GomokuChess>;

    fn into_iter(self) -> Self::IntoIter {
        self.chess.iter()
    }
}

impl GomokuData {
    pub fn iter(&self) -> std::slice::Iter<'_, GomokuChess> {
        self.into_iter()
    }
}
