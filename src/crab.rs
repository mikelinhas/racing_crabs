use crate::crabrace::{Screen, BLOCK_SIZE_X, BLOCK_SIZE_Y};
use termion::color;

#[derive(Debug)]
pub struct Crab {
    pub pos_x: usize,
    pub pos_y: usize,
    pub finished: bool,
}

impl Crab {
    pub fn new(pos_y: usize) -> Crab {
        return Crab {
            pos_x: 0,
            pos_y: pos_y,
            finished: false,
        };
    }

    pub fn update_to_fix(&mut self, screen: &mut Screen, pos_x: usize, pos_y: usize) -> bool {
        self.update(screen, self.pos_x, self.pos_y, true);
        if !self.reached_the_end(screen, pos_x, pos_y) {
            self.pos_x = pos_x;
            self.pos_y = pos_y;
            self.update(screen, pos_x, pos_y, false);
            return true;
        } else {
            self.update(screen, self.pos_x, self.pos_y, false);
            return false;
        }
    }

    pub fn update(&self, screen: &mut Screen, pos_x: usize, pos_y: usize, erase: bool) {
        Self::update_block_vec(
            screen,
            pos_x,
            pos_y,
            Self::get_color(self),
            Self::crab_pattern(self),
            erase,
        );
    }

    pub fn reached_the_end(&self, screen: &mut Screen, pos_x: usize, pos_y: usize) -> bool {
        let pattern = self.crab_pattern();

        for i in 0..pattern.len() {
            for j in 0..pattern[i].len() {
                if pattern[i][j] == "X" {
                    let mut reached_the_end = false;

                    reached_the_end |= screen[pos_y + i * BLOCK_SIZE_Y][pos_x + j * BLOCK_SIZE_X]
                        != String::from(" ");
                    reached_the_end |= screen[pos_y + i * BLOCK_SIZE_Y]
                        [pos_x + j * BLOCK_SIZE_X + 1]
                        != String::from(" ");

                    if reached_the_end {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn get_color(&self) -> color::Rgb {
        return color::Rgb(180, 0, 0);
    }

    pub fn crab_pattern<'a>(&self) -> Vec<Vec<&'a str>> {
        return vec![vec!["X"], vec!["X"]];
    }

    pub fn update_block_vec(
        screen: &mut Screen,
        pos_x: usize,
        pos_y: usize,
        color: termion::color::Rgb,
        blocks: Vec<Vec<&str>>,
        erase: bool,
    ) {
        for i in 0..blocks.len() {
            for j in 0..blocks[i].len() {
                if blocks[i][j] == "X" {
                    Self::update_block(
                        screen,
                        pos_x + j * BLOCK_SIZE_X,
                        pos_y + i * BLOCK_SIZE_Y,
                        color,
                        erase,
                    );
                }
            }
        }
    }

    pub fn update_block(
        screen: &mut Screen,
        pos_x: usize,
        pos_y: usize,
        color: termion::color::Rgb,
        erase: bool,
    ) {
        if erase {
            screen[pos_y][pos_x] = String::from(" ");
            screen[pos_y][pos_x + 1] = String::from(" ");
        } else {
            screen[pos_y][pos_x] = format!("{}{}", color::Fg(color), "█");
            screen[pos_y][pos_x + 1] = format!("{}{}", color::Fg(color), "█");
        }
    }

    pub fn move_to_position(&mut self, screen: &mut Screen, position: usize) {
        if !self.update_to_fix(screen, position, self.pos_y) {
            self.finished = true;
        }
    }
}
