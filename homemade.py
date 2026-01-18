"""
Some example classes for people who want to create a homemade bot.

With these classes, bot makers will not have to implement the UCI or XBoard interfaces themselves.
"""
import uuid
import chess
from chess.engine import PlayResult, Limit
import random
from lib.engine_wrapper import MinimalEngine
from lib.types import MOVE, HOMEMADE_ARGS_TYPE
import logging
from typing import Optional, Type
from types import TracebackType
import requests
import json

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
        self.server_url = "http://localhost:2832"
        self.game_id = f"mystic_{uuid.uuid4().hex[:8]}"
        self.initialized = False

    def _sync_to_rust(self, board: chess.Board):
        if not self.initialized:
            payload = {
                "game_id": self.game_id,
                "current_fen": board.fen(),
                "history": [m.uci() for m in board.move_stack]
            }
            try:
                res = requests.post(
                    f"{self.server_url}/game", json=payload, timeout=5)
                if res.status_code in [200, 201]:
                    self.initialized = True
            except Exception as e:
                logger.error(f"Initalization error: {e}")
        else:
            if board.move_stack:
                last_move = board.peek().uci()
                try:
                    requests.post(f"{self.server_url}/game/move",
                                  json={"game_id": self.game_id,
                                        "mov": last_move},
                                  timeout=2)
                except Exception as e:
                    logger.error(f"Move sync error: {e}")

    def search(self, board: chess.Board, time_limit: Limit, ponder: bool, draw_offered: bool, root_moves: MOVE) -> PlayResult:
        self._sync_to_rust(board)
        timeRemaining = time_limit.white_clock if board.turn == chess.WHITE else time_limit.black_clock
        if timeRemaining is None:
            timeRemaining = 60
        payload = {
            "game_id": self.game_id,
            "time_left_ms": int(timeRemaining * 1000),
            "update_state": True
        }

        try:
            res = requests.post(
                f"{self.server_url}/game/best", json=payload, timeout=time_limit.time)
            data = res.json()
            move_uci = data.get("best_move")
            info = {
                "score": chess.engine.PovScore(chess.engine.Cp(data.get("eval", 0)), board.turn),
                "nodes": data.get("nodes", 0),
                "depth": data.get("depth", 0)
            }

            logger.info(
                f"[{self.game_id}] Move: {move_uci} | Eval: {info['score']} | Depth: {info['depth']}")
            return PlayResult(chess.Move.from_uci(move_uci), None, info=info, draw_offered=draw_offered)

        except Exception as e:
            logger.error(f"Search failed: {e}")
            return PlayResult(random.choice(list(board.legal_moves)), None)

    def __exit__(self, exc_type: Optional[Type[BaseException]],
                    exc_value: Optional[BaseException],
                    traceback: Optional[TracebackType]) -> None:
        """Exit context and allow engine to shutdown nicely if there was no exception."""
        if exc_type is None:
            self.ping()
            self.quit()
        self.engine.__exit__(exc_type, exc_value, traceback)
        requests.delete(f"{self.server_url}/game",
                        params={"game_id": self.game_id}, timeout=1)
