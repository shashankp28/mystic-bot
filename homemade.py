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
import time
from notebooks.lib import boardToBitMap, bitMapFile, extractMove
import os


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

    def __init__(self, *args, **kwargs):
        directory_path = "./tmp"
        fileName = random.randint( 1000000, 9999999 )
        os.makedirs(directory_path, exist_ok=True)
        super().__init__(*args, **kwargs)
        self.executable = "./target/release/mystic-bot"
        self.writePath = f"{directory_path}/{fileName}.json"
        self.chessBoard = chess.Board()
        self.timeout = 60 # Not used yet

        # Run the bot process in the background and wait for 'Mystic Bot Ready' to appear
        self.bot_process = subprocess.Popen(
            [self.executable],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )

        logger.info( "Waiting for Bot to initialize..." )
        while True:
            line = self.bot_process.stdout.readline().strip()
            if "Mystic Bot Ready" in line:
                time.sleep( 0.5 )  # Just for safety
                break
            print( line )
        logger.info("Bot initialized successfully!!")

    def set_chess_board(self, board):
        bitMap = boardToBitMap(board)
        bitMapFile(self.writePath, bitMap, isRead=False)
        self.chessBoard = board

    def get_best_move(self):
        self.bot_process.stdin.write(f"{self.writePath}\n")
        self.bot_process.stdin.flush()
        output = []
        while True:
            line = self.bot_process.stdout.readline().strip()
            if "New Board Saved Successfully" in line: break
            output.append( line )
        move = extractMove( bitMapFile( self.writePath ) )
        return move, '\n'.join( output )

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

        self.set_chess_board( board )
        move, result = self.get_best_move()
        logger.info( result+"\n" )
        logger.info(f"The move played by bot: {move}")
        moveObj = chess.Move.from_uci(move)        

        return PlayResult(moveObj, None, draw_offered=draw_offered)

    def __del__(self):
        self.bot_process.stdin.write("exit\n")
        self.bot_process.stdin.flush()
        self.bot_process.wait()
        logger.info("Bot process terminated!!")
