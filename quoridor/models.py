import itertools as itt
import logging
import numpy as np
from scipy import sparse
import json

DEFAULT_N = 11
INFINITY = float("inf")

logging.basicConfig(level=logging.INFO)
log = logging.getLogger('quoridor')


class InvalidWallError(Exception):
    pass


class Position(object):
    def __init__(self, x=0, y=0, N=DEFAULT_N):
        self.x, self.y, self.N = x, y, N
        if self.x < 0 or self.x >= N or self.y < 0 or self.y >= N:
            raise ValueError('Position out of bounds: ({}, {})'.format(x, y))

    def is_neighbor_to(self, other):
        if abs(self.x - other.x) > 1:
            return False
        if abs(self.y - other.y) > 1:
            return False
        return True

    def shift(self, direction):
        if direction == 'UP':
            self.y -= 1
        elif direction == 'DOWN':
            self.y += 1
        elif direction == 'LEFT':
            self.x -= 1
        elif direction == 'RIGHT':
            self.x += 1
        assert (self.x > 0 and self.x < self.N
                and self.y > 0 and self.y < self.N),\
            'Shift places position out of bounds'
        return self

    @property
    def adjacency_location(self):
        return self.N*self.x + self.y

    def from_adjacency_location(self, loc):
        self.x = loc % self.N
        self.y = loc / self.N
        return self

    def __repr__(self):
        return '<Position({}, {})>'.format(self.x, self.y)

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __ne__(self, other):
        return not self == other


class Wall(object):

    def __init__(self, p1, p2, N=DEFAULT_N):
        self.p1 = p1
        self.p2 = p2
        self.N = N

        if p1 == p2:
            raise InvalidWallError(
                'p1 cannot be the same position as p2')
        if not p1.is_neighbor_to(p2):
            raise InvalidWallError(
                '{} and {} are not neighbors'.format(p1, p2))

        # if (p1.x < 1 or p1.y < 1 or p1.x < 1 or p1.x < 1):
        #     raise InvalidWallError(
        #         'Wall points x, y must be > 0')
        # if (p1.x > N-2 or p1.y > N-2 or p1.x > N-2 or p1.x > N-2):
        #     raise InvalidWallError(
        #         'Wall points x, y must be < {}'.format(N-2))
        # if (p1.x <= 1 or p1.x >= N-1) and self.is_vertical:
        #     raise InvalidWallError(
        #         '{}.x must be > 1 and < {}'.format(p1, self.N-1))
        # if (p2.x <= 1 or p2.x >= N-1) and self.is_vertical:
        #     raise InvalidWallError(
        #         '{}.x must be > 1 and < {}'.format(p2, self.N-1))
        # if (p1.y <= 1 or p1.y >= N-1) and self.is_horizontal:
        #     raise InvalidWallError(
        #         '{}.y must be > 1 and < {}'.format(p1, self.N-1))
        # if (p2.y <= 1 or p2.y >= N-1) and self.is_horizontal:
        #     raise InvalidWallError(
        #         '{}.y must be > 1 and < {}'.format(p2, self.N-1))

    @property
    def is_horizontal(self):
        return self.p1.y == self.p2.y

    @property
    def is_vertical(self):
        return self.p1.x == self.p2.x

    def __repr__(self):
        return 'Wall({}->{})'.format(self.p1, self.p2)


class VerticalWall(Wall):

    def __init__(self, p, N=DEFAULT_N):
        assert p.x > 1 and p.x < N-1,\
            '{}.x must be between 1 and {}'.format(p, N-1)
        p2 = Position(p.x, p.y).shift('DOWN')
        super(VerticalWall, self).__init__(p, p2, N)


class HorizontalWall(Wall):
    def __init__(self, p, N=DEFAULT_N):
        assert p.y > 1 and p.y < N-1,\
            '{}.x must be between 1 and {}'.format(p, N-1)
        p2 = Position(p.x, p.y).shift('RIGHT')
        super(HorizontalWall, self).__init__(p, p2, N)


class Board(object):

    def __init__(self, _board, N=DEFAULT_N):
        self.N = N
        self._board = _board
        self.players = []

    def get_piece_at(self, p):
        for player in self.players:
            if player.pos == p:
                return player.name[:3]
        return ''

    def copy(self):
        b = Board(_board=sparse.lil_matrix(self._board, copy=True))
        b.players = self.players
        return b

    def __repr__(self):
        N = self.N
        ret = ''
        for y in range(N):
            for x in range(N):
                ret += '+'
                if y == 1 or y == N-1:
                    ret += '----'
                elif y > 0 and not self.are_adjacent(
                        Position(x, y), Position(x, y-1)):
                    ret += '####'
                else:
                    ret += '    '
            ret += '+\n '
            for x in range(N):
                icon = self.get_piece_at(Position(x, y)).center(4)
                if x == 0 or x == N-2:
                    ret += icon + '|'
                elif x < N-1 and not self.are_adjacent(
                        Position(x, y), Position(x+1, y)):
                    ret += icon + '#'
                else:
                    ret += icon + ' '
            ret += '\n'
        ret += '+    '*(N+1)
        return ret

    def are_adjacent(self, p1, p2):
        a = p1.adjacency_location
        b = p2.adjacency_location
        return self._board[a, b] != INFINITY

    def dump_adjacency_matrix(self):
        for x in self._board:
            for y in x:
                print ('#' if y else '.'),
            print ''

    def remove_adjacency(self, p1, p2):
        if p1 == p2:
            raise InvalidWallError(
                'cannot remove adjacency from itself: {}'.format(p1))
        log.debug('removing adjacency between {}, {}'.format(p1, p2))
        a = p1.adjacency_location
        b = p2.adjacency_location
        if not self._board[a, b]:
            raise InvalidWallError(
                'positions are already disjoint: {} {}'.format(p1, p2))
        self._board[a, b] = INFINITY
        self._board[b, a] = INFINITY

    def players_have_paths(self):
        for p in self.players:
            assert self.has_path_to(p.pos, p.win_positions),\
                '{} has no path to win positions'.format(p)
        return True

    def insert_wall(self, wall):
        if not self.are_adjacent(wall.p1, wall.p2):
            raise InvalidWallError('Wall not valid {}, {}'.format(
                wall.p1, wall.p2))
        test = self.copy()
        # Add wall on test board
        if wall.is_horizontal:
            test.remove_adjacency(Position(wall.p1.x, wall.p1.y-1), wall.p1)
            test.remove_adjacency(Position(wall.p2.x, wall.p2.y-1), wall.p2)
        elif wall.is_vertical:
            test.remove_adjacency(Position(wall.p1.x-1, wall.p1.y), wall.p1)
            test.remove_adjacency(Position(wall.p2.x-1, wall.p2.y), wall.p2)
        test.players_have_paths()

        log.info('Inserting wall {}'.format(wall))
        if wall.is_horizontal:
            self.remove_adjacency(Position(wall.p1.x, wall.p1.y-1), wall.p1)
            self.remove_adjacency(Position(wall.p2.x, wall.p2.y-1), wall.p2)
        elif wall.is_vertical:
            self.remove_adjacency(Position(wall.p1.x-1, wall.p1.y), wall.p1)
            self.remove_adjacency(Position(wall.p2.x-1, wall.p2.y), wall.p2)

    def is_empty(self, p):
        return p not in [a.pos for a in self.players]

    def has_path_to(self, position, destinations, board=None):
        """thanks be to floyd"""
        dist = sparse.lil_matrix(board or self._board, copy=True)
        dist = sparse.csgraph.floyd_warshall(dist)
        for dest in destinations:
            d = dist[position.adjacency_location, dest.adjacency_location]
            if d < INFINITY:
                return True
        return False


class Player(object):

    def __init__(self, name):
        assert name is not None
        self.name = name
        self.game = None
        self.board = None
        self.pos = None
        self.win_positions = []
        self.walls = 0
        self.direction = None

    def __repr__(self):
        return '<Player({})>'.format(self.name)

    def __eq__(self, other):
        return self.name == other.name

    def __ne__(self, other):
        return not self == other

    def assert_valid_direction(self, direction):
        directions = ['UP', 'DOWN', 'LEFT', 'RIGHT']
        assert direction in directions,\
            'Direction {} not in valid directions: {}'.format(
                direction, directions)

    def asserts_before_turn(self):
        assert self.game, '{} not attached to game'.format(self)
        assert self.game.started,\
            '{} attempted took turn before game started'.format(self)
        assert self.game.turn == self,\
            '{} attempted out of order turn'.format(self)
        assert self.game.started,\
            'Cannot move player, game not started'

    def place_wall(self, wall):
        self.asserts_before_turn()
        assert self.walls > 0, '{}: out of walls'.format(self)
        log.info('{}: insert wall {}'.format(self, wall))
        self.board.insert_wall(wall)
        self.walls -= 1
        self.game.next_turn()

    def move(self, direction, jump=None):
        self.asserts_before_turn()
        self.assert_valid_direction(direction)
        move_p = Position(self.pos.x, self.pos.y)
        move_p.shift(direction)
        assert self.board.are_adjacent(self.pos, move_p),\
            'move: Positions are not adjacent: {}, {}'.format(
                self.pos, move_p)
        if self.board.is_empty(move_p):
            self.pos = move_p
        else:
            jump_p = Position(move_p.x, move_p.y)
            jump_p.shift(direction)
            if self.board.are_adjacent(move_p, jump_p):
                assert not jump or direction == jump,\
                    'Attempt to corner jump when straight jump possible'
                self.pos = jump_p
            else:
                assert jump,\
                    'Straight jump not possible, specify jump direction'
                self.assert_valid_direction(jump)
                jump_p = Position(move_p.x, move_p.y)
                jump_p.shift(jump)
                assert self.board.are_adjacent(move_p, jump_p),\
                    'Specified corner jump not valid'
                self.pos = jump_p
        self.game.next_turn()

    def has_valid_path(self):
        return self.board.has_path_to(self.pos, self.win_positions)


class Game(object):

    def __init__(self, N=DEFAULT_N):
        self.started = False
        self._board = np.zeros((N*N, N*N))
        self._board.fill(INFINITY)
        self._board = sparse.lil_matrix(self._board)
        self.board = Board(self._board)
        self.setup_board()
        self.players = []
        self.board.players = self.players
        self.winner = None
        self.starting_positions = [
            ('DOWN', Position(N/2, 1), [Position(x, N-1) for x in range(N)]),
            ('UP', Position(N/2, N-2), [Position(x, N-1) for x in range(N)]),
            ('RIGHT', Position(1, N/2), [Position(x, N-1) for x in range(N)]),
            ('LEFT', Position(N-2, N/2), [Position(x, N-1) for x in range(N)]),
        ]

    def to_json(self):
        return {
            'ascii': str(self.board),
            'players': {
                p.name: {
                    'walls': p.walls,
                    'x': p.pos.x,
                    'y': p.pos.y,
                    'direction': p.direction
                } for p in self.players
            },
        }

    @property
    def adjacency_matrix(self):
        return [[0 if n == INFINITY else 1 for n in row]
                for row in self._board.toarray()]

    def start(self):
        log.info('Starting game with players: {}'.format(self.players))
        assert len(self.players) % 2 == 0,\
            'Uneven number of players: {}'.format(self.players)
        for player in self.players:
            player.walls = 20/len(self.players)
        self.started = True
        self.turns = itt.cycle(self.players)
        self._turn = None
        self.next_turn()

    def register(self, *players):
        assert not self.started, 'Game already started!'
        for player in players:
            assert len(self.players) < 4,\
                'too many players: {} + {}'.format(self.players, player)
            assert player not in self.players,\
                'player already registered: {}'.format(player)
            self.players.append(player)
            player.game = self
            player.board = self.board
            direction, pos, win_positions = self.starting_positions.pop(0)
            player.direction = direction
            player.pos = pos
            player.win_positions = win_positions
        return self

    def _win(self, p):
        log.info('{} wins'.format(p))
        self.started = False
        self.winner = p

    def resign(self, *players):
        for player in players:
            assert player in self.players,\
                'player not registered: {}'.format(player)
        return self

    def check_win(self):
        for player in self.players:
            if player.pos in player.win_positions:
                self._win(player)
        return self

    def next_turn(self):
        assert self.started, 'Game not started!'
        self.turn = self.turns.next()
        self.check_win()
        log.info('New turn {}'.format(self.turn))
        return self

    def setup_board(self):
        assert not self.started, 'Game already started!'
        N = self.board.N
        for y in range(N):
            for x in range(N):
                p1 = Position(x, y)
                a = p1.adjacency_location
                for xx, yy in [(-1, 0), (0, -1), (1, 0), (0, 1)]:
                    if x+xx < 0 or x+xx >= N or y+yy < 0 or y+yy >= N:
                        continue
                    b = Position(x+xx, y+yy).adjacency_location
                    self._board[a, b] = 1
                    self._board[b, a] = 1

        for i in range(N-1):
            self.board.remove_adjacency(Position(i, 0), Position(i+1, 0))
            self.board.remove_adjacency(Position(i, N-1), Position(i+1, N-1))
            self.board.remove_adjacency(Position(0, i), Position(0, i+1))
            self.board.remove_adjacency(Position(N-1, i), Position(N-1, i+1))
