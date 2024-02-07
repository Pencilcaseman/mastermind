# Mastermind
The game Mastermind implemented in Rust, with optimal algorithms to play the game.

## Compiling and Running
With a modern version of Rust installed, clone the repository and run `cargo run` to run the program.

```sh 
git clone https://github.com/Pencilcaseman/mastermind.git 
cd mastermind 
cargo build # Optional. Cargo run will compile the code
cargo run # Add `--release` to enable optimisations (much faster)
```

## Using the Program 

### Interacting with the Program 

#### From the Command Line 

You can pick a mode from the command line by specifying additional arguments to the program:

```
Usage: mastermind [OPTIONS]

Options:
  -t, --target
  -c, --correctness
  -b, --beat
  -a, --adversarial
  -h, --help         Print help
  -V, --version      Print version
```

Note that to pass arguments via Cargo, you can use `cargo run -- --help`, for example (note the `--` with spaces either side). 

#### From Inside the Program 

There is a menu you can use in the program, with options to select different game modes.

```
 ======= Menu =======
- 1 Play With Target
- 2 Check Correctness
- 3 Beat Game
- 4 Adversarial
>>>
```
Enter the mode you'd like (`2`, for example, to run `Check Correctness`) and press enter. Type `exit` to quit. 

### Game Modes
#### Play with Target 
The user enters a target, and the program outputs the best possible guess you can make at any given stage. You can enter any guess, however, and the game will play out as expected. 

#### Check Correctness

For every possible target set, run the optimal algorithm and check that it does, indeed, find the correct solution. It also records the number of guesses necessary to find the solution.

It is recommended that you only run this mode in `--release` configurations, since it can be quite slow. If I have time, I'll see if I can optimise the algorithm to make it faster.

#### Beat Game 
For use against another player -- make the guess provided by the program and enter the set of pegs returned (black/white or full/partial matches). The program will continue to tell you the optimal move to make until you win the game.

#### Adversarial
For you to play against the computer (on hard mode...). You make guesses and see how quickly you can guess the computer's chosen target.

It's also fun to have the program open in two terminal windows, watching the optimal algorithm (`Beat Game`) play against the optimal 'guess' (`Adversarial`).

(Spoiler: The computer will make your life as difficult as possible (mathematically))


