#![warn(clippy::pedantic, clippy::nursery)]

mod config;
mod game;

use clap::Parser;
use colored::*;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author="Toby Davis", version, about="Mastermind Implemented in Rust", long_about = None)]
struct Args {
    #[arg(short, long, name = "true/false")]
    target: bool,

    #[arg(short, long)]
    correctness: bool,

    #[arg(short, long)]
    beat: bool,

    #[arg(short, long)]
    adversarial: bool,
}

fn main() {
    let args = Args::parse();

    if args.target {
        play_with_target();
    } else if args.correctness {
        check_correctness();
    } else if args.beat {
        beat_game();
    } else if args.adversarial {
        adversarial();
    }

    'outer: loop {
        println!(
            "{}",
            format!(" ======= {} =======", "Menu".truecolor(255, 200, 50))
                .truecolor(200, 50, 255)
                .bold()
        );
        println!(
            "{} {} Play With Target",
            "-".truecolor(100, 100, 100),
            "1".blue().bold(),
        );
        println!(
            "{} {} Check Correctness",
            "-".truecolor(100, 100, 100),
            "2".blue().bold(),
        );
        println!(
            "{} {} Beat Game",
            "-".truecolor(100, 100, 100),
            "3".blue().bold(),
        );
        println!(
            "{} {} Adversarial",
            "-".truecolor(100, 100, 100),
            "4".blue().bold(),
        );

        let choice = loop {
            print!(">>> ");
            if std::io::stdout().flush().is_err() {
                panic!("Failed to flush buffer");
            }

            let mut line = String::new();

            if std::io::stdin().read_line(&mut line).is_ok() {
                if line.trim() == "exit" {
                    break 'outer;
                }

                if let Ok(result) = line.trim().parse::<i64>() {
                    println!("Result: {result}");
                    if (1..=4).contains(&result) {
                        break result;
                    }
                }

                println!(
                    "{}",
                    "Invalid value. Please enter 1, 2, 3, or 4, or 'exit' to quit".red()
                );
            }
        };

        match choice {
            1 => play_with_target(),
            2 => check_correctness(),
            3 => beat_game(),
            4 => adversarial(),
            _ => panic!("Unknown error"),
        }
    }
}

fn play_with_target() {
    println!("{}", "Play with Target".green().bold());

    let mut game = game::Game::new(game::user_input_row());
    let mut open = game::gen_perms();

    println!("{game}\n");

    loop {
        println!(
            "Best Move out of {} Possibilities: {}",
            format!("{}", open.len()).green().bold(),
            game::find_best_move(&open)
        );
        let new_row = game::user_input_row();
        let state = game::compare_rows(&game.target, &new_row);
        open = game::apply_filter(&open, &(new_row, state));
        game.rows.push((new_row, state));
        println!("{game}");

        if state == game::State::Complete {
            break;
        }
    }
}

fn check_correctness() {
    println!("{}", "Check Correctness".green().bold());
    let mut total = 0;
    let mut count = 0;
    let mut min = (
        usize::MAX,
        game::Row {
            items: [
                game::Piece::Red,
                game::Piece::Red,
                game::Piece::Red,
                game::Piece::Red,
            ],
        },
    );

    let mut max = (
        0,
        game::Row {
            items: [
                game::Piece::Red,
                game::Piece::Red,
                game::Piece::Red,
                game::Piece::Red,
            ],
        },
    );

    for item in game::gen_perms() {
        print!("{item}\r");
        if std::io::stdout().flush().is_err() {
            panic!("Unable to flush buffer");
        }

        let rows = game::optimal_solve(&item).rows.len();

        if rows > max.0 {
            max.0 = rows;
            max.1 = item;
        }

        if rows < min.0 {
            min.0 = rows;
            min.1 = item;
        }

        total += rows;
        count += 1;
    }

    println!("\nTotal Rows            : {total}");
    println!("Number of Games       : {count}");
    println!("Maximum number of rows: {} => {}", max.1, max.0);
    println!("Minimum number of rows: {} => {}", min.1, min.0);
    println!(
        "Average number of rows: {}\n",
        (total as f32) / (count as f32)
    );
}

fn beat_game() {
    println!("{}", "Beat the Game".green().bold());

    let mut open = game::gen_perms();

    loop {
        if open.is_empty() {
            println!("{}", "No possible solutions".red().bold());
            break;
        }

        let best_move = game::find_best_move(&open);
        println!(
            "{} {}",
            format!(
                "Best Move from {} possibilities:",
                format!("{}", open.len()).truecolor(255, 200, 100).bold()
            )
            .blue()
            .bold(),
            best_move
        );

        println!("{}", "Outcome:".red().bold());
        let state = game::user_input_state();

        open = game::apply_filter(&open, &(best_move, state));

        if state == game::State::Complete {
            break;
        }
    }

    println!();
}

fn adversarial() {
    println!("{}", "Adversarial".green().bold());

    let mut open = game::gen_perms();
    let states = game::gen_states();

    let mut game = game::Game::empty();

    println!("{game}\n");

    loop {
        if open.is_empty() {
            println!("{}", "No possible solutions".red().bold());
            break;
        }

        let new_row = game::user_input_row();

        // Find the state which leaves the open set as large as possible
        let adversarial_state = states
            .iter()
            .max_by_key(|&&state| game::count_apply_filter(&open, &(new_row, state)))
            .unwrap();

        if open.len() == 1 && open[0] == new_row {
            game.target = new_row;
        }

        open = game::apply_filter(&open, &(new_row, *adversarial_state));

        game.rows.push((new_row, *adversarial_state));

        println!("{game}");

        if open.is_empty() {
            break;
        }
    }

    println!();
}
