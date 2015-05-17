import logging
import unittest

logging.basicConfig(level=logging.DEBUG)
logger = logging.getLogger(__name__)

from quoridor.models import Position, Game, Board


class TestQuoridor(unittest.TestCase):

    def test_position_neighbor(self):
        p1 = Position(3, 4)
        p2 = Position(4, 4)
        p3 = Position(3, 5)
        self.assertTrue(p1.is_neighbor_to(p2))
        self.assertTrue(p1.is_neighbor_to(p3))

    def test_adjacency(self):
        g = Game()
        p1 = Position(0, 0)
        p2 = Position(1, 0)
        self.assertTrue(g.board.are_adjacent(p1, p2))
