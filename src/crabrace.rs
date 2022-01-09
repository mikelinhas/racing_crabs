use std::io::{stdout, Read, Write};
use std::thread;
use std::time::{Duration, SystemTime};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{async_stdin, clear, color, cursor, style};

use crate::crab::Crab;

pub const BLOCK_SIZE_X: usize = 2;
pub const BLOCK_SIZE_Y: usize = 1;

const BOARD_SIZE_X: usize = 50 * BLOCK_SIZE_X;
const BOARD_SIZE_Y: usize = 10 * BLOCK_SIZE_Y;

pub const BORDER_SIZE_X: usize = BLOCK_SIZE_X;
pub const BORDER_SIZE_Y: usize = BLOCK_SIZE_Y;

pub const GAME_SIZE_X: usize = BOARD_SIZE_X + BORDER_SIZE_X * 3;
pub const GAME_SIZE_Y: usize = BOARD_SIZE_Y + BORDER_SIZE_Y * 2;

pub type Screen = Vec<Vec<String>>;

pub struct Crabrace {
    game: Screen,
    crab: Crab,
    crab_opponent: Crab,
    changed: bool,
    your_crab_won: bool,
    race_finished: bool,
}

impl Crabrace {
    pub fn new() -> Crabrace {
        Crabrace {
            game: vec![vec![String::from(" "); GAME_SIZE_X]; GAME_SIZE_Y],
            crab: Crab::new(BORDER_SIZE_Y + BLOCK_SIZE_Y * 5),
            crab_opponent: Crab::new(BORDER_SIZE_Y + BLOCK_SIZE_Y * 2),
            changed: true,
            your_crab_won: false,
            race_finished: false,
        }
    }

    pub fn play(&mut self) {
        let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        let mut stdin = async_stdin().bytes();
        let mut time_at_last_frame = SystemTime::now();

        self.init();
        self.update();

        while !self.race_finished {
            let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
            let mut pos_x = 1;
            let mut pos_y = 1;

            if usize::from(terminal_width) > GAME_SIZE_X {
                pos_x = (usize::from(terminal_width) - GAME_SIZE_X) / 2;
            }

            if usize::from(terminal_height) > GAME_SIZE_Y {
                pos_y = (usize::from(terminal_height) - GAME_SIZE_Y) / 2;
            }

            // update
            if time_at_last_frame.elapsed().unwrap().as_millis() > 500 {
                time_at_last_frame = SystemTime::now();
                if !self.update() {
                    break;
                }
            }

            // print
            if self.changed {
                write!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();
                self.print_border();
                self.print(pos_x, pos_y);
                self.changed = false;
            }

            // key listener
            let c = stdin.next();
            match c {
                Some(Ok(b'q')) | Some(Ok(3)) => break, // q or Ctrl + c for quit
                Some(Ok(67)) => {
                    self.crab
                        .move_to_position(&mut self.game, self.crab.pos_x + BLOCK_SIZE_X);
                    self.changed = true
                } // arrow right
                _ => (),
            }

            thread::sleep(Duration::from_millis(10));
        }

        if self.your_crab_won {
            self.you_won()
        } else {
            self.game_over();
        }
    }

    fn init(&mut self) {
        self.crab.move_to_position(&mut self.game, BLOCK_SIZE_X);
    }

    fn update(&mut self) -> bool {
        let new_position = self.crab_opponent.pos_x + BLOCK_SIZE_X;
        self.crab_opponent
            .move_to_position(&mut self.game, new_position);

        if self.crab.finished {
            self.race_finished = true;
            self.your_crab_won = true;
            return false;
        } else if self.crab_opponent.finished {
            self.race_finished = true;
            return false;
        } else {
            self.changed = true;
        }

        return true;
    }

    fn you_won(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let (terminal_width, terminal_height) = termion::terminal_size().unwrap();

        write!(
            stdout,
            "{}{}{}{}",
            clear::All,
            cursor::Goto((terminal_width - 10) / 2, terminal_height / 2),
            style::Reset,
            "Yay! YOU WON!"
        )
        .unwrap();

        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(2000));
    }

    fn game_over(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let (terminal_width, terminal_height) = termion::terminal_size().unwrap();

        write!(
            stdout,
            "{}{}{}{}",
            clear::All,
            cursor::Goto((terminal_width - 10) / 2, terminal_height / 2),
            style::Reset,
            "ALMOST! The other crab was faster."
        )
        .unwrap();

        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(2000));
    }

    fn print_border(&mut self) {
        let color = color::Rgb(0, 102, 102);

        // vertical Border
        for i in 0..GAME_SIZE_Y {
            self.game[i][0] = format!("{}{}", color::Fg(color), "█");
            self.game[i][1] = format!("{}{}", color::Fg(color), "█");
            self.game[i][GAME_SIZE_X - BORDER_SIZE_X] = format!("{}{}", color::Fg(color), "█");
            self.game[i][GAME_SIZE_X - BORDER_SIZE_X + 1] = format!("{}{}", color::Fg(color), "█");
        }

        // horizontal border
        for i in 0..GAME_SIZE_X {
            self.game[0][i] = format!("{}{}", color::Fg(color), "█");
            self.game[GAME_SIZE_Y - 1][i] = format!("{}{}", color::Fg(color), "█");
        }
    }

    fn print(&mut self, pos_x: usize, pos_y: usize) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        for i in 0..GAME_SIZE_Y {
            write!(stdout, "{}", cursor::Goto(pos_x as u16, (pos_y + i) as u16)).unwrap();
            for j in 0..GAME_SIZE_X {
                write!(stdout, "{}", self.game[i][j]).unwrap();
            }
        }

        stdout.flush().unwrap();
    }
}
