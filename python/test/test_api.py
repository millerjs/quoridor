import logging
import unittest
import json
from quoridor.models import (
    Position, Game, VerticalWall, HorizontalWall, Player
)
import requests

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

url = 'http://localhost:8000{}'


def get(route):
    r = requests.get(url.format(route))
    print r.text
    r.raise_for_status()
    try:
        return r.json()
    except:
        return r.text


def post(route, data={}, **kwargs):
    kwargs.update(data)
    r = requests.post(url.format(route), data=json.dumps(kwargs),
                      headers={'Content-Type': 'application/json'})
    print r.text
    r.raise_for_status()
    try:
        return r.json()
    except:
        return r.text


class TestQuoridorAPI(unittest.TestCase):

    def test_game_creation(self):
        print post('/game', description='test game')['id']

    def test_player_register(self):
        game = post('/game')['id']
        print post('/game/{}/register'.format(game),
                   name='User1', description='basically deep blue')

    def test_get_state(self):
        game = post('/game')['id']
        post('/game/{}/register'.format(game), name='A')
        post('/game/{}/register'.format(game), name='B')
        post('/game/{}/start'.format(game))
        print get('/game/{}/state'.format(game))

    def test_ascii(self):
        game = post('/game')['id']
        post('/game/{}/register'.format(game), name='A')
        post('/game/{}/register'.format(game), name='B')
        print get('/game/{}/ascii'.format(game))

    def test_repeated_player_register(self):
        game = post('/game')['id']
        print post('/game/{}/register'.format(game), {'name': 'A'})
        with self.assertRaises(Exception):
            print post('/game/{}/register'.format(game), {'name': 'A'})

    def test_simple_game(self):
        game = post('/game')['id']
        a = post('/game/{}/register'.format(game), {'name': 'A'})['player_id']
        b = post('/game/{}/register'.format(game), {'name': 'B'})['player_id']
        post('/game/{}/start'.format(game))
        post('/game/{}/{}/move/{}'.format(game, a, 'DOWN'))
        get('/game/{}/state'.format(game))
        for i in range(7):
            post('/game/{}/{}/move/{}'.format(game, b, 'UP'))
            post('/game/{}/{}/move/{}'.format(game, a, 'DOWN'))
        print get('/game/{}/ascii'.format(game))
        with self.assertRaises(Exception):
            post('/game/{}/{}/move/{}'.format(game, a, 'DOWN'))
        print get('/game/{}/ascii'.format(game))

    def test_jump_straight(self):
        game = post('/game')['id']
        a = post('/game/{}/register'.format(game), {'name': 'A'})['player_id']
        b = post('/game/{}/register'.format(game), {'name': 'B'})['player_id']
        post('/game/{}/start'.format(game))
        for i in range(3):
            post('/game/{}/{}/move/{}'.format(game, a, 'DOWN'))
            post('/game/{}/{}/move/{}'.format(game, b, 'UP'))
        post('/game/{}/{}/move/{}'.format(game, a, 'DOWN'))
        post('/game/{}/{}/move/{}'.format(game, b, 'UP'))
        print get('/game/{}/ascii'.format(game))
        print get('/game/{}/state'.format(game))

    def test_place_wall(self):
        game = post('/game')['id']
        a = post('/game/{}/register'.format(game), {'name': 'A'})['player_id']
        b = post('/game/{}/register'.format(game), {'name': 'B'})['player_id']
        post('/game/{}/start'.format(game))
        post('/game/{}/{}/wall/horizontal/5/5'.format(game, a))
        for i in range(3):
            post('/game/{}/{}/move/{}'.format(game, b, 'UP'))
            post('/game/{}/{}/move/{}'.format(game, a, 'DOWN'))
        post('/game/{}/{}/move/{}'.format(game, b, 'UP'))
        with self.assertRaises(Exception):
            post('/game/{}/{}/move/{}'.format(game, a, 'DOWN'))
        post('/game/{}/{}/move/{}'.format(game, a, 'LEFT'))
        print get('/game/{}/ascii'.format(game))
