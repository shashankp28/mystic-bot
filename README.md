# Mystic Chess Bot

A simple Chess Bot using Rust

To clone the repository run:
```bash
git clone https://github.com/shashankp28/mystic-bot.git
```

## Bot Algorithm

### Bit-Board Representation

```rust
#[derive(Serialize)]
pub struct Board {

    // Flattended Matrix representation of 8x8 Chess Board, with `a1` at the Top-Left
    // Bit is 1 if the corresponding piece is at corresponding index else 0
    // The black and white parts of the boards are concatenated in 64+64 = 128 bits
    // The MSB part corresponds to black and LSB part corresponds to white
    // The below representation based on
    // Video: https://www.youtube.com/watch?v=w4FFX_otR-4&pp=ygUSbWFraW5nIGEgY2hlc3MgYm90
    pub rooks: u128,
    pub knights: u128,
    pub bishops: u128,
    pub queens: u128,
    pub kings: u128,
    pub pawns: u128,

    // 1 bit, whether the board has an en-passant
    // It is not possible for a board to have multiple en-passants at the same time!
    // ( X, X, X, en_passant, Black o-o, Black o-o-o, White o-o, White o-o-o )
    pub metadata: u8,
}
```

## Display Chess Board

This project allows you to generate a chessboard image with pieces displayed on it. The pieces and the board are represented as images stored in a specified directory. You can customize the arrangement of the pieces by modifying the board configuration.

### Features

- Customizable chessboard layout.
- Generates a PNG image of the chessboard with pieces.
- Uses the Pillow library for image processing.

### Requirements

You can install the required library using pip:

```bash
pip install -r requirements.txt
```

### Usage

Using the `analysis.ipynb` is straight-forward