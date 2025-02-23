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
from notebooks.lib import oldstyle_fen
import os
from typing import Optional, Type
from types import TracebackType
from collections import defaultdict


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
            my_time = time_limit.white_clock if isinstance(
                time_limit.white_clock, int) else 0
            my_inc = time_limit.white_inc if isinstance(
                time_limit.white_inc, int) else 0
        else:
            my_time = time_limit.black_clock if isinstance(
                time_limit.black_clock, int) else 0
            my_inc = time_limit.black_inc if isinstance(
                time_limit.black_inc, int) else 0

        possible_moves = root_moves if isinstance(
            root_moves, list) else list(board.legal_moves)

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
        if not os.path.exists("./tmp/"):
            os.mkdir("./tmp/")
        super().__init__(*args, **kwargs)
        self.executable = "./target/release/mystic-bot"
        self.chessBoard = chess.Board()
        self.timeout = 60  # Not used yet
        self.timeRemaining = 0
        self.historyFile = f"./tmp/{random.randint(10000000, 99999999)}.txt"
        self.fenVals = []

        # Run the bot process in the background and wait for 'Mystic Bot Ready' to appear
        self.bot_process = subprocess.Popen(
            [self.executable],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )

        logger.info("Waiting for Bot to initialize...")
        while True:
            line = self.bot_process.stdout.readline().strip()
            if "Mystic Bot Ready" in line:
                time.sleep(0.5)  # Just for safety
                break
            print(line)
        logger.info("Bot initialized successfully!!")

    def set_chess_board(self, board):
        self.chessBoard = board

    def save_fen_vals(self):
        with open(self.historyFile, "w+") as f:
            for fen in self.fenVals:
                f.write(f"{fen}\n")

    def get_best_move(self):
        msTimeRemainPerMove = int(self.timeRemaining*1000 / 40)
        timeToBeUsed = min(5000, msTimeRemainPerMove)
        logger.info(f"Time used: {timeToBeUsed}")
        self.bot_process.stdin.write(
            f'"{oldstyle_fen(self.chessBoard)}" {timeToBeUsed}\n')
        self.bot_process.stdin.flush()
        output = []
        while True:
            line = self.bot_process.stdout.readline().strip()
            output.append(line)
            if "Best next move" in line:
                break
        move = output[-1].split()[-1]
        return move, '\n'.join(output)

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

        self.fenVals.append(board.fen())
        self.save_fen_vals()

        self.set_chess_board(board)
        self.timeRemaining = time_limit.white_clock if board.turn == chess.WHITE else time_limit.black_clock
        if self.timeRemaining is None:
            self.timeRemaining = 10 * 60
        move, result = self.get_best_move()
        logger.info(result+"\n")
        logger.info(f"The move played by bot: {move}")
        moveObj = chess.Move.from_uci(move)

        board.push(moveObj)
        self.fenVals.append(board.fen())
        self.save_fen_vals()

        return PlayResult(moveObj, None, draw_offered=draw_offered)

    def terminate_bot(self):
        """
        Explicitly terminate the bot process.
        """
        if self.bot_process:
            self.bot_process.stdin.write("exit\n")
            self.bot_process.stdin.flush()
            self.bot_process.wait()
            logger.info("Bot process terminated!!")
            self.bot_process = None
            os.remove(self.historyFile)

    def __del__(self):
        """
        Ensure bot process is terminated when the object is deleted.
        """
        if self.bot_process:
            self.terminate_bot()

    def __exit__(self, exc_type: Optional[Type[BaseException]],
                 exc_value: Optional[BaseException],
                 traceback: Optional[TracebackType]) -> None:
        """Exit context and allow engine to shutdown nicely if there was no exception."""
        if exc_type is None:
            self.ping()
            self.quit()
        self.engine.__exit__(exc_type, exc_value, traceback)
        if self.bot_process:
            self.terminate_bot()
