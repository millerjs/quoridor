import logging
import unittest
import json
from quoridor.models import (
    Position, Game, VerticalWall, HorizontalWall, Player
)

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class TestQuoridor(unittest.TestCase):

    def test_position_neighbor(self):
        p1 = Position(3, 4)
        p2 = Position(4, 4)
        p3 = Position(3, 5)
        self.assertTrue(p1.is_neighbor_to(p2))
        self.assertTrue(p1.is_neighbor_to(p3))

    def test_adjacency(self):
        g = Game()
        p1 = Position(1, 1)
        p2 = Position(2, 1)
        p3 = Position(2, 2)
        self.assertTrue(g.board.are_adjacent(p1, p2))
        self.assertFalse(g.board.are_adjacent(p1, p3))

    def test_wall_insert(self):
        g = Game()
        w = VerticalWall(Position(2, 1))
        g.board.insert_wall(w)
        print g.board
        self.assertFalse(g.board.are_adjacent(
            Position(1, 1), Position(2, 1)))

    def test_jump(self):
        g = Game()
        a, b = Player('A'), Player('B')
        g.register(a, b)
        g.start()
        for i in range(3):
            a.move('DOWN')
            b.move('UP')
        a.move('DOWN')
        self.assertRaises(Exception, a.move, 'UP', 'LEFT')
        self.assertRaises(Exception, a.move, 'UP', 'RIGHT')
        self.assertRaises(Exception, a.move, 'UP', 'DOWN')
        b.move('UP')
        print g.board

    def test_jump_left(self):
        g = Game()
        a, b = Player('A'), Player('B')
        g.register(a, b).start()
        a.pos = Position(3, 5)
        b.pos = Position(3, 4)
        g.board.insert_wall(HorizontalWall(Position(3, 4)))
        self.assertRaises(
            Exception,
            a.move, 'UP', 'UP')
        a.move('UP', 'LEFT')
        print g.board

    def test_jump_down(self):
        g = Game()
        a, b = Player('A'), Player('B')
        g.register(a, b).start()
        a.pos = Position(2, 5)
        b.pos = Position(3, 5)
        g.board.insert_wall(VerticalWall(Position(4, 4)))
        self.assertRaises(
            Exception,
            a.move, 'RIGHT', 'RIGHT')
        a.move('RIGHT', 'DOWN')
        print g.board

    def test_path_exists(self):
        g = Game()
        a = Player('A')
        b = Player('B')
        g.register(a, b)
        g.start()
        for i in range(1, g.board.N-2, 2):
            g.board.insert_wall(HorizontalWall(Position(i, 2)))
        print g.board
        self.assertTrue(a.has_valid_path())
        with self.assertRaises(AssertionError):
            g.board.insert_wall(VerticalWall(Position(9, 1)))
        g.board.remove_adjacency(Position(8, 1), Position(9, 1))
        print g.board
        self.assertFalse(a.has_valid_path())
