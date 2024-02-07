#![warn(clippy::pedantic, clippy::nursery)]

use crate::config;
use colored::Colorize;
use rayon::prelude::*;
use std::io::Write;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
    None,
}

pub const PIECES: [Piece; 6] = [
    Piece::Red,
    Piece::Green,
    Piece::Blue,
    Piece::Yellow,
    Piece::Black,
    Piece::White,
];

impl Piece {
    pub const fn id(self) -> usize {
        match self {
            Self::Red => 0,
            Self::Green => 1,
            Self::Blue => 2,
            Self::Yellow => 3,
            Self::Black => 4,
            Self::White => 5,
            Self::None => usize::MAX,
        }
    }

    pub const fn from_id(id: u8) -> Self {
        match id {
            0 => Self::Red,
            1 => Self::Green,
            2 => Self::Blue,
            3 => Self::Yellow,
            4 => Self::Black,
            5 => Self::White,
            _ => panic!("Invalid ID"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Unknown,
    Known(i32, i32), // Full, partial
    Complete,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Row {
    pub items: [Piece; config::NUM_ELEMENTS],
}

#[derive(Debug)]
pub struct Game {
    pub target: Row,
    pub rows: Vec<(Row, State)>,
}

fn apply_board(text: &str) -> colored::ColoredString {
    text.on_truecolor(
        config::BOARD_COLOR.0,
        config::BOARD_COLOR.1,
        config::BOARD_COLOR.2,
    )
}

impl Default for State {
    fn default() -> Self {
        Self::Unknown
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use colored::*;
        let c = apply_board("●");
        match self {
            Self::Red => write!(f, "{}", c.red()),
            Self::Green => write!(f, "{}", c.green()),
            Self::Blue => write!(f, "{}", c.blue()),
            Self::Yellow => write!(f, "{}", c.yellow()),
            Self::Black => write!(f, "{}", c.black()),
            Self::White => write!(f, "{}", c.white()),
            Self::None => write!(f, "{}", "?".white()),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "[    ]"),
            Self::Complete => write!(f, "[DONE]"),
            Self::Known(full, partial) => {
                let mut result = String::new();
                result.push_str(&format!("[{}", "*".repeat(*full as usize).black()));
                result.push_str(&format!("{}", "*".repeat(*partial as usize).white()));
                result.push_str(&format!(
                    "{}]",
                    " ".repeat(config::NUM_ELEMENTS - *full as usize - *partial as usize)
                ));

                write!(f, "{}", apply_board(&result))
            }
        }
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = " ".to_string();

        for item in &self.items {
            result.push_str(&format!("{item} "));
        }

        write!(f, "{}", &apply_board(&result))
    }
}

impl Game {
    pub fn new(target: Row) -> Self {
        Game {
            target,
            rows: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Game {
            target: Row {
                items: [Piece::None; config::NUM_ELEMENTS],
            },
            rows: Vec::new(),
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = apply_board(&format!("{} target ", self.target)).to_string();
        result.push_str("\n");
        result.push_str(&apply_board("=================").to_string());
        result.push_str("\n");

        for row in &self.rows {
            result.push_str(&apply_board(&format!("{} {} ", row.0, row.1)).to_string());
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

pub fn gen_perms() -> Vec<Row> {
    let mut count = [0u8; config::NUM_ELEMENTS];
    let mut result = Vec::with_capacity(config::NUM_ELEMENTS.pow(PIECES.len() as u32));

    while (count[config::NUM_ELEMENTS - 1] as usize) < PIECES.len() {
        result.push(Row {
            items: std::array::from_fn(|i| Piece::from_id(count[i])),
        });

        count[0] += 1;

        let mut i = 0;
        while i < config::NUM_ELEMENTS - 1 && (count[i] as usize) >= PIECES.len() {
            count[i] -= PIECES.len() as u8;
            count[i + 1] += 1;
            i += 1
        }
    }

    result
}

pub fn gen_states() -> Vec<State> {
    let mut result = Vec::with_capacity(15);
    for a in 0..=config::NUM_ELEMENTS {
        for b in 0..=config::NUM_ELEMENTS - a {
            result.push(State::Known(a as i32, b as i32))
        }
    }

    result
}

pub fn user_input_row() -> Row {
    for (index, val) in PIECES.iter().enumerate() {
        print!("{}", apply_board(&format!("  {index} {val}  ")));
    }

    println!();

    // Read line as input (NUM_ELEMENTS digits)
    let mut line = String::new();
    loop {
        line.clear();
        print!(">>> ");

        if let None = std::io::stdout().flush().ok() {
            panic!("Unable to flush buffer");
        }

        if let Some(bytes) = std::io::stdin().read_line(&mut line).ok() {
            if bytes - 1 != config::NUM_ELEMENTS {
                println!(
                    "{}",
                    format!("Input requires {} elements", config::NUM_ELEMENTS).red()
                );
            } else {
                let bytes = line.trim().as_bytes();
                let mut valid = true;
                let result: [Piece; config::NUM_ELEMENTS] = std::array::from_fn(|i| {
                    if let Some(result) = PIECES.iter().enumerate().find_map(|(index, piece)| {
                        if bytes[i] >= b'0' && (bytes[i] - b'0') as usize == index {
                            Some(piece)
                        } else {
                            None
                        }
                    }) {
                        *result
                    } else {
                        println!(
                            "{}",
                            format!(
                                "Invalid value '{}'",
                                char::from_u32(bytes[i] as u32).unwrap()
                            )
                            .red()
                        );
                        valid = false;
                        Piece::Red
                    }
                });

                if valid {
                    return Row { items: result };
                }
            }
        } else {
            println!("{}", "The input was invalid. Please try again.".red());
        }
    }
}

pub fn user_input_state() -> State {
    println!("'F' => Full/Black\n'P' => Partial/White");

    loop {
        print!(">>> ");
        if let None = std::io::stdout().flush().ok() {
            panic!("Unable to flush buffer");
        }

        let mut line = String::new();
        if let Some(_) = std::io::stdin().read_line(&mut line).ok() {
            let mut full = 0;
            let mut partial = 0;
            let mut valid = true;
            for b in line.trim().bytes() {
                match b {
                    b'F' | b'f' => full += 1,
                    b'P' | b'p' => partial += 1,
                    _ => {
                        println!(
                            "{}",
                            format!("Invalid character '{}'", char::from_u32(b as u32).unwrap())
                                .red()
                        );
                        valid = false;
                    }
                }
            }

            if valid {
                if full == 4 {
                    return State::Complete;
                } else if full + partial <= 4 {
                    return State::Known(full, partial);
                }
            }
        }
    }
}

pub fn compare_rows(target: &Row, src: &Row) -> State {
    if *target == *src {
        return State::Complete;
    }

    let mut count = [0u8; PIECES.len()];
    let mut seen = [false; PIECES.len()];

    let black = target
        .items
        .iter()
        .zip(src.items.iter())
        .enumerate()
        .filter_map(|(index, (a, b))| {
            if a == b {
                seen[index] = true;
                Some(0)
            } else {
                count[a.id()] += 1;
                None
            }
        })
        .count();

    let white = src
        .items
        .iter()
        .enumerate()
        .filter_map(|(index, p)| {
            let id = p.id();
            if !seen[index] && count[id] > 0 {
                count[id] -= 1;
                Some(0)
            } else {
                None
            }
        })
        .count();

    State::Known(black as i32, white as i32)
}

pub fn apply_filter(open: &[Row], filter: &(Row, State)) -> Vec<Row> {
    open.iter()
        .filter_map(|row| {
            if compare_rows(row, &filter.0) == filter.1 {
                Some(row.clone())
            } else {
                None
            }
        })
        .collect()
}

#[inline(always)]
pub fn count_apply_filter(open: &[Row], filter: &(Row, State)) -> usize {
    open.iter()
        .filter(|row| compare_rows(row, &filter.0) == filter.1)
        .count()
}

pub fn find_best_move(open: &[Row]) -> Row {
    let states = gen_states();
    // open.par_iter()
    gen_perms()
        .par_iter()
        // .iter()
        .min_by_key(|guess| {
            states
                .iter()
                .map(|state| count_apply_filter(open, &(**guess, *state)))
                .max()
                .unwrap()
        })
        .unwrap()
        .clone()
}

// pub fn find_adversarial_move(open: &[Row]) -> State {
//     // Find the State which produces the largst possible search space
//     let states = gen_states();
//     states.iter().max_by_key(|state|
//         count_apply_filter(open, )
//     )
// }

pub fn optimal_solve(target: &Row) -> Game {
    let mut game = Game::new(*target);
    let mut open = gen_perms();

    loop {
        let new_row = find_best_move(&open);
        let state = compare_rows(&game.target, &new_row);
        game.rows.push((new_row, state));
        open = apply_filter(&open, &(new_row, state));

        if state == State::Complete {
            break;
        }
    }

    game
}
