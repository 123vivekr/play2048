# play2048

Play 2048 on the command line. This game is implemented in rust-lang.

# How to play?
1. Clone this repo
```
git clone git@github.com:123vivekr/play2048.git
```

2. Enter `play2048` directory
```
cd play2048
```

3. Start the CLI game!
```
cargo run <board_dimension> [target]
```
Parameters:
- `board_dimension`: Dimensions of the board. Example `4` for a 4x4 board.
- `target`: Target to achieve to win the game. (Must be a power of 2). Example `2048`. (Defaults to `2048`)

Example: `cargo run 4 2048`