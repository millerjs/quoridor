import math

DEFAULT_N = 9


class InvalidWallError(Exception):
    pass


class Position(object):
    def __init__(self, x, y, N=DEFAULT_N):
        self.x = x
        self.y = y
        self.N = N

        if self.x < 0:
            raise ValueError('off board! x = {} < 0'.format(x))
        if self.y < 0:
            raise ValueError('off board! y = {} < 0'.format(y))
        if self.x >= N:
            raise ValueError('off board! x = {} >= {}'.format(x, N))
        if self.y >= N:
            raise ValueError('off board! y = {} >= {}'.format(y, N))

    def is_neighbor_to(self, other):
        if abs(self.x - other.x) > 1:
            return False
        if abs(self.y - other.y) > 1:
            return False
        return True

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
        if p1.x == 0 or p1.x >= N:
            raise InvalidWallError(
                'p1.x ({}) must be > 0 and < {}'.format(p1, self.N))
        if p2.x == 0 or p2.x >= N:
            raise InvalidWallError(
                'p2.x ({}) must be > 0 and < {}'.format(p2, self.N))
        if p1.y == 0 or p1.y >= N:
            raise InvalidWallError(
                'p1.y ({}) must be > 0 and < {}'.format(p1, self.N))
        if p2.y == 0 or p2.y >= N:
            raise InvalidWallError(
                'p2.y ({}) must be > 0 and < {}'.format(p2, self.N))

    @property
    def is_horizontal(self):
        return self.p1.y == self.p2.y

    @property
    def is_vertical(self):
        return self.p1.x == self.p2.x

    def __repr__(self):
        return 'Wall({}->{})'.format(self.p1, self.p2)


class Board(object):

    def __init__(self, _board):
        self._N = len(_board)
        self.N = int(math.sqrt(self._N))
        self._board = _board

    def __repr__(self):
        N = self.N
        ret = ''
        for y in range(N):
            for x in range(N):
                ret += '+'
                if y < N-1 and not self.are_adjacent(
                        Position(x, y), Position(x, y+1)):
                    ret += '----'
                else:
                    ret += '    '
            ret += '+\n '
            for x in range(N):
                if x < N-1 and not self.are_adjacent(
                        Position(x, y), Position(x+1, y)):
                    ret += '   |'
                else:
                    ret += '    '
            ret += '\n'
        ret += '+    '*(N+1)
        return ret

    def are_adjacent(self, p1, p2):
        a = p1.adjacency_location
        b = p2.adjacency_location
        return self._board[a][b]

    def add_wall(self, wall):
        if not self.are_adjacent(wall.p1, wall.p2):
            raise RuntimeError('')

    def dump_adjacency_matrix(self):
        for x in self._board:
            for y in x:
                print ('#' if y else '.'),
            print ''

    def remove_adjacency(self, p1, p2):
        if p1 == p2:
            raise InvalidWallError(
                'cannot remove adjacency from itself: {}'.format(p1))
        a = p1.adjacency_location
        b = p2.adjacency_location
        self._board[a][b] = False
        self._board[b][a] = False


class Game(object):
    __tablename__ = 'games'

    def __init__(self, N=DEFAULT_N):
        self._board = [[False for x in range(N*N)]
                       for y in range(N*N)]
        self.board = Board(self._board)
        for y in range(N):
            for x in range(N):
                p1 = Position(x, y)
                a = p1.adjacency_location
                for xx in range(max(0, x-1), min(N, x+1)):
                    for yy in range(max(0, y-1), min(N, y+1)):
                        p2 = Position(xx, yy)
                        self._board[a][p2.adjacency_location] = True
                        self._board[p2.adjacency_location][a] = True

        self.board.remove_adjacency(Position(2, 3), Position(2, 4))
        self.board.remove_adjacency(Position(4, 4), Position(5, 4))
