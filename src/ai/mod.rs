use crate::game::gomoku::{GomokuChess, GomokuData};

pub async fn next_chess(data: &GomokuData) -> GomokuChess {
    let current = data.current;
    let size = data.size;
    let mut x = 0;
    let mut y = 0;
    let mut max = 0;
    for i in 0..size {
        for j in 0..size {
            if let None = data.check_grid(i, j) {
                let mut count = 0;
                for (dx, dy) in &[(1, 0), (0, 1), (1, 1), (1, -1)] {
                    let mut c = 1;
                    for k in 1..5 {
                        let x = i as i32 + dx * k;
                        let y = j as i32 + dy * k;
                        if x < 0 || y < 0 || x >= size as i32 || y >= size as i32 {
                            break;
                        }
                        if let Some(grid) = data.check_grid(x as u32, y as u32) {
                            if grid == current {
                                c += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    for k in 1..5 {
                        let x = i as i32 - dx * k;
                        let y = j as i32 - dy * k;
                        if x < 0 || y < 0 || x >= size as i32 || y >= size as i32 {
                            break;
                        }
                        if let Some(grid) = data.check_grid(x as u32, y as u32) {
                            if grid == current {
                                c += 1;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    count = count.max(c);
                }
                if count > max {
                    max = count;
                    x = i;
                    y = j;
                }
            }
        }
    }
    GomokuChess { x, y, color: data.current }
}
