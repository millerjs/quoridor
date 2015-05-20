import itertools as itt
import logging
import numpy as np
from scipy import sparse
import string
import random

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

    def copy(self, direction=None):
        p = Position(self.x, self.y)
        if direction:
            p.shift(direction)
        return p

    def shift(self, direction):
        if direction == 'UP':
            self.y -= 1
        elif direction == 'DOWN':
            self.y += 1
        elif direction == 'LEFT':
            self.x -= 1
        elif direction == 'RIGHT':
            self.x += 1
        else:
            raise RuntimeError('Unknown direction {}'.format(direction))
        assert (self.x > 0 and self.x < self.N
                and self.y > 0 and self.y < self.N),\
            'Shift places position out of bounds'
        return self

    @property
    def adjacency_location(self):
        return self.N*self.x + self.y

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

    @property
    def is_horizontal(self):
        return self.p1.y == self.p2.y

    @property
    def is_vertical(self):
        return self.p1.x == self.p2.x

    def __repr__(self):
        return '{}({}->{})'.format(self.__class__.__name__, self.p1, self.p2)


class VerticalWall(Wall):

    def __init__(self, p, N=DEFAULT_N):
        assert p.x > 1 and p.x < N-1,\
            '{}.x must be between 1 and {}'.format(p, N-1)
        super(VerticalWall, self).__init__(p, p.copy('DOWN'), N)

    def add_to_board(self, board):
        board.remove_adjacency(self.p1.copy('LEFT'), self.p1)
        board.remove_adjacency(self.p2.copy('LEFT'), self.p2)


class HorizontalWall(Wall):
    def __init__(self, p, N=DEFAULT_N):
        assert p.y > 1 and p.y < N-1,\
            '{}.x must be between 1 and {}'.format(p, N-1)
        super(HorizontalWall, self).__init__(p, p.copy('RIGHT'), N)

    def add_to_board(self, board):
        board.remove_adjacency(self.p1.copy('UP'), self.p1)
        board.remove_adjacency(self.p2.copy('UP'), self.p2)


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
        a, b = p1.adjacency_location, p2.adjacency_location
        return (self._board[a, b] != INFINITY)

    def remove_adjacency(self, p1, p2):
        log.debug('removing adjacency between {}, {}'.format(p1, p2))
        a, b = p1.adjacency_location, p2.adjacency_location
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
        test = self.copy()
        wall.add_to_board(test)
        test.players_have_paths()
        log.info('Inserting wall {}'.format(wall))
        wall.add_to_board(self)

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

    def __init__(self, name, description=None):
        self.game, self.board, self.pos, self.direction = [None]*4
        assert name is not None, 'Player must have a name'
        self.description = description
        self.name = name
        self.win_positions = []
        self.walls = 0

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
        move_p = self.pos.copy(direction)
        assert self.board.are_adjacent(self.pos, move_p),\
            'move: Positions are not adjacent: {}, {}'.format(
                self.pos, move_p)
        if self.board.is_empty(move_p):
            self.pos = move_p
        else:
            jump_p = move_p.copy(direction)
            if self.board.are_adjacent(move_p, jump_p):
                assert not jump or direction == jump,\
                    'Attempt to corner jump when straight jump possible'
                self.pos = jump_p
            else:
                assert jump,\
                    'Straight jump not possible, specify jump direction'
                self.assert_valid_direction(jump)
                jump_p = move_p.copy(jump)
                assert self.board.are_adjacent(move_p, jump_p),\
                    'Specified corner jump not valid'
                self.pos = jump_p
        self.game.next_turn()

    def has_valid_path(self):
        return self.board.has_path_to(self.pos, self.win_positions)


class Game(object):

    def __init__(self, description='', N=DEFAULT_N):
        self.game_id = ''.join(random.choice(
            string.ascii_lowercase + string.digits) for _ in range(5))
        self.started = False
        self.description = description
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
        log.info('New game: {}'.format(self))

    def __repr__(self):
        return '<Game({})>'.format(self.game_id)

    def get_player(self, name):
        assert Player(name) in self.players,\
            'No player named {}'.format(name)
        return [p for p in self.players if p.name == name][0]

    def to_json(self):
        return {
            'started': self.started,
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
        assert not self.started,\
            '{} already started'.format(self)
        assert len(self.players),\
            'No players: {}'.format(self)
        assert len(self.players) % 2 == 0,\
            'Uneven number of players: {}'.format(self.players)
        log.info('Starting game with players: {}'.format(self.players))
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
