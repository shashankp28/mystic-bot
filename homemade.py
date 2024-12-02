"""
Some example classes for people who want to create a homemade bot.

With these classes, bot makers will not have to implement the UCI or XBoard interfaces themselves.
"""
import chess
from chess.engine import PlayResult, Limit
import random
from lib.engine_wrapper import MinimalEngine
from lib.types import MOVE, HOMEMADE_ARGS_TYPE
import logging
import json
import subprocess


# Use this logger variable to print messages to the console or log files.
# logger.info("message") will always print "message" to the console or log file.
# logger.debug("message") will only print "message" if verbose logging is enabled.
logger = logging.getLogger(__name__)


class ExampleEngine(MinimalEngine):
    """An example engine that all homemade engines inherit."""

    pass


# Bot names and ideas from tom7's excellent eloWorld video

class RandomMove(ExampleEngine):
    """Get a random move."""

    def search(self, board: chess.Board, *args: HOMEMADE_ARGS_TYPE) -> PlayResult:
        """Choose a random move."""
        return PlayResult(random.choice(list(board.legal_moves)), None)


class Alphabetical(ExampleEngine):
    """Get the first move when sorted by san representation."""

    def search(self, board: chess.Board, *args: HOMEMADE_ARGS_TYPE) -> PlayResult:
        """Choose the first move alphabetically."""
        moves = list(board.legal_moves)
        moves.sort(key=board.san)
        return PlayResult(moves[0], None)


class FirstMove(ExampleEngine):
    """Get the first move when sorted by uci representation."""

    def search(self, board: chess.Board, *args: HOMEMADE_ARGS_TYPE) -> PlayResult:
        """Choose the first move alphabetically in uci representation."""
        moves = list(board.legal_moves)
        moves.sort(key=str)
        return PlayResult(moves[0], None)


class ComboEngine(ExampleEngine):
    """
    Get a move using multiple different methods.

    This engine demonstrates how one can use `time_limit`, `draw_offered`, and `root_moves`.
    """

    def search(self, board: chess.Board, time_limit: Limit, ponder: bool, draw_offered: bool, root_moves: MOVE) -> PlayResult:
        """
        Choose a move using multiple different methods.

        :param board: The current position.
        :param time_limit: Conditions for how long the engine can search (e.g. we have 10 seconds and search up to depth 10).
        :param ponder: Whether the engine can ponder after playing a move.
        :param draw_offered: Whether the bot was offered a draw.
        :param root_moves: If it is a list, the engine should only play a move that is in `root_moves`.
        :return: The move to play.
        """
        if isinstance(time_limit.time, int):
            my_time = time_limit.time
            my_inc = 0
        elif board.turn == chess.WHITE:
            my_time = time_limit.white_clock if isinstance(time_limit.white_clock, int) else 0
            my_inc = time_limit.white_inc if isinstance(time_limit.white_inc, int) else 0
        else:
            my_time = time_limit.black_clock if isinstance(time_limit.black_clock, int) else 0
            my_inc = time_limit.black_inc if isinstance(time_limit.black_inc, int) else 0

        possible_moves = root_moves if isinstance(root_moves, list) else list(board.legal_moves)

        if my_time / 60 + my_inc > 10:
            # Choose a random move.
            move = random.choice(possible_moves)
        else:
            # Choose the first move alphabetically in uci representation.
            possible_moves.sort(key=str)
            move = possible_moves[0]
        return PlayResult(move, None, draw_offered=draw_offered)


class MysticBot(ExampleEngine):
    
    """
    Get a move using multiple different methods.

    This engine demonstrates how one can use `time_limit`, `draw_offered`, and `root_moves`.
    """

    def bitMapFile(self, fileName, bitMap=None, isRead=True):
        if isRead:
            with open(fileName, "r+") as f: 
                bitMap = json.load(f)
                return bitMap
        else:
            assert bitMap is not None, "No bitMap provided  to save!"
            with open(fileName, "w+") as f: 
                json.dump(bitMap, f, indent=2)

    def extractMove(self, bitMap):
        isWhite = ( bitMap[ 'metadata' ] >> 8 ) & 1
        latestMove = bitMap[ 'latest_move' ]
        isKingSideCastle = ( latestMove >> 13 ) & 1
        if isKingSideCastle != 0:
            return 'e1g1' if isWhite else 'e8g8'
        isQueenSideCastle = ( latestMove >> 12 ) & 1
        if isQueenSideCastle != 0:
            return 'e1c1' if isWhite else 'e8c8'
        source = ( latestMove >> 6 ) & 63
        destination = latestMove & 63
        sourceX, sourceY = source%8, source//8
        destinationX, destinationY = destination%8, destination//8

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

    def boardToBitMap(self, board):

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
            effIndex = 64*isBlack + ( 63 - index )
            boardBitMap[ pieceStructMap[ piece.symbol().lower() ] ] |= 1 << effIndex

        metadata = 0
        metadata |= board.fullmove_number << 16
        metadata |= board.halfmove_clock << 9
        metadata |= board.turn << 8
        en_passant_square = board.ep_square
        metadata |= bool( en_passant_square ) << 7
        if en_passant_square:
            column_number = chess.square_file(en_passant_square)
            metadata |= column_number << 4
        metadata |= board.has_kingside_castling_rights(0) << 3
        metadata |= board.has_queenside_castling_rights(0) << 2
        metadata |= board.has_kingside_castling_rights(1) << 1
        metadata |= board.has_queenside_castling_rights(1) << 0
        boardBitMap[ 'metadata' ] = metadata

        return boardBitMap

    def set_fen_position(self, fen):
        chessBoard = chess.Board(fen)
        self.bitMapFile(self.writeFile, self.boardToBitMap(chessBoard), isRead=False)
        self.chessBoard = chessBoard

    def get_best_move(self):
        result = subprocess.run(
            [self.executable, self.writeFile],
            check=True,
            capture_output=True,
            text=True
        )
        move = self.extractMove(self.bitMapFile(self.writeFile))
        return move, result

    def search(self, board: chess.Board, time_limit: Limit, ponder: bool, draw_offered: bool, root_moves: MOVE) -> PlayResult:
        """
        Choose a move using multiple different methods.

        :param board: The current position.
        :param time_limit: Conditions for how long the engine can search (e.g. we have 10 seconds and search up to depth 10).
        :param ponder: Whether the engine can ponder after playing a move.
        :param draw_offered: Whether the bot was offered a draw.
        :param root_moves: If it is a list, the engine should only play a move that is in `root_moves`.
        :return: The move to play.
        """
        self.executable = "./target/release/mystic-bot"
        self.writeFile = f"./tmp.json"
        self.chessBoard = None

        self.set_fen_position( board.fen() )
        move, _ = self.get_best_move()
        moveObj = chess.Move.from_uci(move)        

        return PlayResult(moveObj, None, draw_offered=draw_offered)