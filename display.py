import os
import argparse
from PIL import Image
import json
from default.custom import customBoard

binaryPieceMap = {
    "rooks": [ "BR", "WR" ],
    "knights": [ "BN", "WN" ],
    "bishops": [ "BB", "WB" ],
    "queens": [ "BQ", "WQ" ],
    "king": [ "BK", "WK" ],
    "pawns": [ "BP", "WP" ],
}


class ChessBoard:

    def __init__(self, binaryBoardFile=None, custom=False):

        self.board = self.getDefaultBoard()
        self.enPassant = 0
        self.castlingRights = 63
        if custom:
            self.board, self.enPassant, self.castlingRights = customBoard
        elif binaryBoardFile:
            with open(binaryBoardFile, 'r+') as f:
                data = json.load(f)
            board = [[None]*8 for _ in range(8)]
            for key in data:
                if key not in binaryPieceMap:
                    continue
                curr_value = data[key]
                for isWhite in [ 1, 0 ]:
                    for i in range(8):
                        for j in range(8):
                            if curr_value % 2:
                                board[7-i][7-j] = binaryPieceMap[key][ isWhite ]
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
            "rooks": 0,
            "knights": 0,
            "bishops": 0,
            "queens": 0,
            "king": 0,
            "pawns": 0,
            "en_passant": 0,
            "castling_rights": 0
        }

    def getDefaultBinaryBoard(self):
        return {
            "rooks": 2388925415139424862208,
            "knights": 1222240910071333650432,
            "bishops": 666676860038909263872,
            "queens": 296300826683959672832,
            "king": 148150413341979836416,
            "pawns": 1204203524907878590709760,
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
        index = 127
        for isWhite in [ 0, 1 ]:
            for i in range(8):
                for j in range(8):
                    for key, value in binaryPieceMap.items():
                        binaryVals[key] = binaryVals[key] | ( value[ isWhite ] == self.board[i][j] ) << index
                    index -= 1
        binaryVals['en_passant'] = self.enPassant
        binaryVals['castling_rights'] = self.castlingRights
        with open(fileName, 'w+') as f:
            json.dump(binaryVals, f)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(
        description="Save the chessboard image to a file")
    parser.add_argument(
        '-bi', '--binaryIn',
        type=str,
        required=False,
        help='The binary json file to be used to generate the board (eg. default/board.json)'
    )
    parser.add_argument(
        '-ci', '--customIn',
        type=bool,
        required=False,
        help='The custom board to be used as input ( edit. default/custom.py )'
    )
    parser.add_argument(
        '-bo', '--binaryOut',
        type=str,
        required=False,
        help='The binary json file to dump the board (eg. gen/board.json)'
    )
    parser.add_argument(
        '-io', '--imageOut',
        type=str,
        required=False,
        help='The png filename to save the chessboard image (e.g., gen/output.png)'
    )

    args = parser.parse_args()
    print(args)
    board = ChessBoard(args.binaryIn, args.customIn)
    board.boardToBinary(args.binaryOut)
    board.display(args.imageOut)

#####################################################################
