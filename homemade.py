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
from typing import Optional, Type
from types import TracebackType
import requests


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
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.chessBoard = chess.Board()
        self.timeout = 60  # Not used
        self.timeRemaining = 0
        self.fenVals = []
        self.server_url = "http://localhost:8080"

    def set_chess_board(self, board: chess.Board):
        self.chessBoard = board

    def get_best_move(self):

        payload = {
            "current_fen": self.chessBoard.fen(),
            "history": [],
            "time_left_ms": int(self.timeRemaining * 1000),
        }

        print("Payload:", payload)
        res = requests.get(f"{self.server_url}/eval", json=payload)
        data = res.json()
        print("Response:", data)

        if res.status_code != 200 or data["best_move"] is None:
            raise Exception(f"Failed to get best move: {data}")

        return data["best_move"], data

    def search(self, board: chess.Board, time_limit: Limit, ponder: bool, draw_offered: bool, root_moves: MOVE) -> PlayResult:
        self.set_chess_board(board)

        self.timeRemaining = time_limit.white_clock if board.turn == chess.WHITE else time_limit.black_clock
        if self.timeRemaining is None:
            self.timeRemaining = 60  # 60 seconds

        move_uci, debug_info = self.get_best_move()
        move_obj = chess.Move.from_uci(move_uci)
        board.push(move_obj)

        logger.info(f"Move chosen: {move_uci}\n{debug_info}")
        return PlayResult(move_obj, None, draw_offered=draw_offered)

    def terminate_bot(self):
        try:
            res = requests.delete(
                f"{self.server_url}/game?game_id={self.game_id}")
            if res.status_code == 200:
                logger.info("Game session cleaned up.")
        except Exception as e:
            logger.warning(f"Failed to delete game session: {e}")

    def __del__(self):
        self.terminate_bot()

    def __exit__(self, exc_type: Optional[Type[BaseException]],
                 exc_value: Optional[BaseException],
                 traceback: Optional[TracebackType]) -> None:
        if exc_type is None:
            self.ping()
            self.quit()
        self.engine.__exit__(exc_type, exc_value, traceback)
        self.terminate_bot()
