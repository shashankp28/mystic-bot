import os
import argparse
from PIL import Image

# <<=============  Custom Board =====================>>
customBoard = [
    ['BR', 'BN', 'BB', 'BQ', 'BK', 'BB', 'BN', 'BR'],
    ['BP', 'BP', 'BP', 'BP', 'BP', 'BP', 'BP', 'BP'],
    [None, None, None, None, None, None, None, None, ],
    [None, None, None, None, None, None, None, None, ],
    [None, None, None, None, None, None, None, None, ],
    [None, None, None, None, None, None, None, None, ],
    ['WP', 'WP', 'WP', 'WP', 'WP', 'WP', 'WP', 'WP'],
    ['WR', 'WN', 'WB', 'WQ', 'WK', 'WB', 'WN', 'WR'],
]
# <<======================================================>>


############   DO NOT EDIT BELOW THIS LINE!!!    ##################

## <<=============  New Board ( For Reference) =====================>>
newBoard = [
    ['BR', 'BN', 'BB', 'BQ', 'BK', 'BB', 'BN', 'BR'],
    ['BP', 'BP', 'BP', 'BP', 'BP', 'BP', 'BP', 'BP'],
    [None, None, None, None, None, None, None, None, ],
    [None, None, None, None, None, None, None, None, ],
    [None, None, None, None, None, None, None, None, ],
    [None, None, None, None, None, None, None, None, ],
    ['WP', 'WP', 'WP', 'WP', 'WP', 'WP', 'WP', 'WP'],
    ['WR', 'WN', 'WB', 'WQ', 'WK', 'WB', 'WN', 'WR'],
]
# <<======================================================>>


squareSize = 150
pieceSize = int(0.7 * squareSize)

pieceImages = {file.split('.')[0]: Image.open(f'img/{file}').convert('RGBA')
               for file in os.listdir('img/') if 'board' not in file
               }


for key in pieceImages:
    pieceImages[key] = pieceImages[key].resize(
        (pieceSize, pieceSize), Image.Resampling.LANCZOS)


def displayBoard(chessBoard, filename):
    boardImage = Image.open('img/board.png').convert('RGBA')
    for row in range(8):
        for col in range(8):
            piece = chessBoard[row][col]
            if piece:
                pieceImage = pieceImages[piece]
                position = (col * squareSize, row * squareSize)
                boardImage.paste(pieceImage, position, pieceImage.split()[3])
    boardImage.save(filename )


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description="Save the chessboard image to a file")
    parser.add_argument(
        '-o', '--output',
        type=str,
        required=True,
        help='The png filename to save the chessboard image (e.g., output.png)'
    )

    args = parser.parse_args()
    displayBoard(customBoard, args.output)

#####################################################################
