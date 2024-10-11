import os
import argparse
from PIL import Image
import json

binaryPieceMap = {
    "white_rooks": "WR",
    "white_knights": "WN",
    "white_bishops": "WB",
    "white_queens": "WQ",
    "white_king": "WK",
    "white_pawns": "WP",
    "black_rooks": "BR",
    "black_knights": "BN",
    "black_bishops": "BB",
    "black_queens": "BQ",
    "black_king": "BK",
    "black_pawns": "BP",
}


class ChessBoard:

    def __init__(self, binaryBoardFile=None):

        self.board = self.getDefaultBoard()
        self.enPassant = 0
        self.castlingRights = 63

        if binaryBoardFile:
            with open(binaryBoardFile, 'r+') as f:
                data = json.load(f)
            board = [[None]*8 for _ in range(8)]
            for key in data:
                if key not in binaryPieceMap:
                    continue
                curr_value = data[key]
                for i in range(8):
                    for j in range(8):
                        if curr_value % 2:
                            board[i][j] = binaryPieceMap[key]
                        curr_value = curr_value >> 1
            self.board = board
            self.enPassant = data['en_passant']
            self.castlingRights = data['castling_rights']

    def getDefaultBoard(self):
        return [
            ['WR', 'WN', 'WB', 'WK', 'WQ', 'WB', 'WN', 'WR'],
            ['WP', 'WP', 'WP', 'WP', 'WP', 'WP', 'WP', 'WP'],
            [None, None, None, None, None, None, None, None, ],
            [None, None, None, None, None, None, None, None, ],
            [None, None, None, None, None, None, None, None, ],
            [None, None, None, None, None, None, None, None, ],
            ['BP', 'BP', 'BP', 'BP', 'BP', 'BP', 'BP', 'BP'],
            ['BR', 'BN', 'BB', 'BK', 'BQ', 'BB', 'BN', 'BR'],
        ]

    def getEmptyBinaryBoard(self):
        return {
            "white_rooks": 0,
            "white_knights": 0,
            "white_bishops": 0,
            "white_queens": 0,
            "white_king": 0,
            "white_pawns": 0,
            "black_rooks": 0,
            "black_knights": 0,
            "black_bishops": 0,
            "black_queens": 0,
            "black_king": 0,
            "black_pawns": 0,
            "en_passant": 0,
            "castling_rights": 0
        }

    def getDefaultBinaryBoard(self):
        return {
            "white_rooks": 0,
            "white_knights": 0,
            "white_bishops": 0,
            "white_queens": 0,
            "white_king": 0,
            "white_pawns": 0,
            "black_rooks": 0,
            "black_knights": 0,
            "black_bishops": 0,
            "black_queens": 0,
            "black_king": 0,
            "black_pawns": 0,
            "en_passant": 0,
            "castling_rights": 63
        }

    def getReverseMap(self, abbr):
        for key, value in binaryPieceMap.items():
            if abbr == value:
                return key
        return None

    def display(self, fileName):
        if not fileName:
            return
        squareSize = 150
        pieceSize = int(0.7 * squareSize)
        pieceImages = {
            file.split('.')[0]: Image.open(f'img/{file}').convert('RGBA')
            for file in os.listdir('img/') if 'board' not in file
        }

        for key in pieceImages:
            pieceImages[key] = pieceImages[key].resize(
                (pieceSize, pieceSize), Image.Resampling.LANCZOS)

        boardImage = Image.open('img/board.png').convert('RGBA')
        for row in range(8):
            for col in range(8):
                piece = self.board[7-row][7-col]
                if piece:
                    pieceImage = pieceImages[piece]
                    position = (col * squareSize, row * squareSize)
                    boardImage.paste(pieceImage, position,
                                     pieceImage.split()[3])
        boardImage.save(fileName)

    def boardToBinary(self, fileName):
        if not fileName:
            return
        binaryVals = self.getEmptyBinaryBoard()
        for i in range(8):
            for j in range(8):
                index = i*8 + j
                for key, value in binaryPieceMap.items():
                    binaryVals[key] = binaryVals[key] | (
                        value == self.board[i][j]) << index
        binaryVals['en_passant'] = self.enPassant
        binaryVals['castling_rights'] = self.castlingRights
        with open(fileName, 'w+') as f:
            json.dump(binaryVals, f)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description="Save the chessboard image to a file")
    parser.add_argument(
        '-bi', '--input',
        type=str,
        required=False,
        help='The binary json file to be used to generate the board (eg. board.json)'
    )
    parser.add_argument(
        '-bo', '--output',
        type=str,
        required=False,
        help='The binary json file to dump the board (eg. board.json)'
    )
    parser.add_argument(
        '-io', '--image',
        type=str,
        required=False,
        help='The png filename to save the chessboard image (e.g., output.png)'
    )

    args = parser.parse_args()
    print(args)
    board = ChessBoard(args.input)
    board.boardToBinary(args.output)
    board.display(args.image)

#####################################################################
