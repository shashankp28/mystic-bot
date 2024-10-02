# Mystic Chess Bot

A simple Chess Bot using Rust

To clone the repository run:
```bash
git clone https://github.com/shashankp28/mystic-bot.git
```

## Bot Algorithm

TODO

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

1. Place your piece images and the chessboard background image in the `img` directory

2. Modify the `customBoard` variable in `display.py` to customize the arrangement of pieces on the chessboard. Default board configuration is as follows:

```python
customBoard = [
    ['BR', 'BN', 'BB', 'BQ', 'BK', 'BB', 'BN', 'BR'],
    ['BP', 'BP', 'BP', 'BP', 'BP', 'BP', 'BP', 'BP'],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    ['WP', 'WP', 'WP', 'WP', 'WP', 'WP', 'WP', 'WP'],
    ['WR', 'WN', 'WB', 'WQ', 'WK', 'WB', 'WN', 'WR'],
]
```

3. Run the script from the command line with the output filename as an argument:

```
python display.py -o output.png
```

4. Replace `output.png` with your desired output filename

## Piece Abbreviations

| Abbreviation | Piece        |
| ------------ | ------------ |
| BR           | Black Rook   |
| BN           | Black Knight |
| BB           | Black Bishop |
| BQ           | Black Queen  |
| BK           | Black King   |
| BP           | Black Pawn   |
| WR           | White Rook   |
| WN           | White Knight |
| WB           | White Bishop |
| WQ           | White Queen  |
| WK           | White King   |
| WP           | White Pawn   |
