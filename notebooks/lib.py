import chess
import random
import json
from IPython.display import display, SVG

def random_move_game(board, num_moves=20):
    for _ in range(num_moves):
        legal_moves = list(board.legal_moves)
        if not legal_moves:
            break
        move = random.choice(legal_moves)
        board.push(move)
    return board


def numPieces(bitMap):

   def sum_bits(num):
      return bin(num).count('1')

   return sum_bits(bitMap["rooks"]) + \
       sum_bits(bitMap["knights"]) + \
       sum_bits(bitMap["bishops"]) + \
       sum_bits(bitMap["queens"]) + \
       sum_bits(bitMap["kings"]) + \
       sum_bits(bitMap["pawns"])


def getPosFromNotation(notation: str, isWhite: bool):
    horizontalPos = ord(notation[1]) - ord('1')
    verticalPos = ord(notation[0]) - ord('a')
    return 1 << ((64*(not isWhite)) + (63 - (horizontalPos*8 + verticalPos)))


def isSameColorPiecePresent(board, pos, isWhiteMove: bool):
   for pieces in board:
      if board[pieces] & (1 << (64*(not isWhiteMove) + pos)):
         return True
   return False


def isOppositeColorPiecePresent(board, pos, isWhiteMove: bool):
    return isSameColorPiecePresent(board, pos, not isWhiteMove)


def isMovePossible(board, pos, isWhiteMove: bool):
   return not (pos >= 64 or pos < 0 or isSameColorPiecePresent(board, pos, isWhiteMove))


def boardToBitMap(board):

   pieceStructMap = {
       "r": "rooks",
       "n": "knights",
       "b": "bishops",
       "k": "kings",
       "q": "queens",
       "p": "pawns",
   }

   boardBitMap = {
       "rooks": 0,
       "knights": 0,
       "bishops": 0,
       "queens": 0,
       "kings": 0,
       "pawns": 0,
       "metadata": 0,
       "latest_move": 0
   }

   for index, piece in board.piece_map().items():
      isBlack = piece.symbol().islower()
      effIndex = 64*isBlack + (63 - index)
      boardBitMap[pieceStructMap[piece.symbol().lower()]] |= 1 << effIndex

   metadata = 0
   metadata |= board.fullmove_number << 16
   metadata |= board.halfmove_clock << 9
   metadata |= board.turn << 8
   en_passant_square = board.ep_square
   metadata |= bool(en_passant_square) << 7
   if en_passant_square:
      column_number = chess.square_file(en_passant_square)
      metadata |= column_number << 4
   metadata |= board.has_kingside_castling_rights(0) << 3
   metadata |= board.has_queenside_castling_rights(0) << 2
   metadata |= board.has_kingside_castling_rights(1) << 1
   metadata |= board.has_queenside_castling_rights(1) << 0
   boardBitMap['metadata'] = metadata

   return boardBitMap


def bitMapFile(fileName, bitMap=None, isRead=True):
   if isRead:
      with open(fileName, "r+") as f:
         bitMap = json.load(f)
         return bitMap
   else:
      assert bitMap is not None, "No bitMap provided  to save!"
      with open(fileName, "w+") as f:
         json.dump(bitMap, f, indent=2)


def bitMapToBoard(bitMap):
   chessPieceMap = {
       "rooks": chess.ROOK,
       "knights": chess.KNIGHT,
       "bishops": chess.BISHOP,
       "kings": chess.KING,
       "queens": chess.QUEEN,
       "pawns": chess.PAWN
   }

   chessBoard = chess.Board()
   chessBoard.clear_board()

   for key, value in bitMap.items():
      if key not in ['metadata', 'latest_move']:
         for index in range(127, -1, -1):
            isPresent = value & 1 << index
            if isPresent:
               isWhite = index <= 63
               effIndex = 63 - (index - (not isWhite) * 64)
               chessPiece = chess.Piece(chessPieceMap[key], isWhite)
               chessBoard.set_piece_at(effIndex, chessPiece)

   castlingFen, metadata = '', bitMap['metadata']
   chessBoard.fullmove_number = metadata >> 16
   chessBoard.halfmove_clock = metadata >> 9 & 127
   chessBoard.turn = bool(metadata & 1 << 8)
   enPassantSquareExists = metadata & 1 << 7
   if enPassantSquareExists:
      rowNum = 5 if chessBoard.turn else 2
      chessBoard.ep_square = chess.square(metadata >> 4 & 7, rowNum)
   if metadata & 1 << 1:
       castlingFen += 'K'
   if metadata & 1 << 0:
       castlingFen += 'Q'
   if metadata & 1 << 3:
       castlingFen += 'k'
   if metadata & 1 << 2:
       castlingFen += 'q'
   chessBoard.set_castling_fen(castlingFen)

   return chessBoard


def display_board(board, size=800):
    svg_board = chess.svg.board(board, size=size)
    display(SVG(svg_board))


def extractMove(bitMap):
   isWhite = (bitMap['metadata'] >> 8) & 1
   latestMove = bitMap['latest_move']
   isKingSideCastle = (latestMove >> 13) & 1

   if isKingSideCastle != 0:
      return 'e1g1' if not isWhite else 'e8g8'
   isQueenSideCastle = ( latestMove >> 12 ) & 1
   if isQueenSideCastle != 0:
      return 'e1c1' if not isWhite else 'e8c8'
   source = (latestMove >> 6) & 63
   destination = latestMove & 63
   sourceX, sourceY = source % 8, source//8
   destinationX, destinationY = destination % 8, destination//8

   sourceLetters = chr(sourceX + ord('a')) + str(sourceY+1)
   destinationLetters = chr(destinationX + ord('a')) + str(destinationY+1)

   # Check for pawn promotion
   isPawnPromotion = (latestMove >> 16) & 1
   promotionPiece = ''
   if isPawnPromotion:
      promotionType = (latestMove >> 14) & 3
      if promotionType == 0:
         promotionPiece = 'Q'
      elif promotionType == 1:
         promotionPiece = 'R'
      elif promotionType == 2:
         promotionPiece = 'B'
      elif promotionType == 3:
         promotionPiece = 'N'
      return f"{sourceLetters}{destinationLetters}{promotionPiece}"

   return f"{sourceLetters}{destinationLetters}"
