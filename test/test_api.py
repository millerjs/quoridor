import logging
import unittest
import json
from quoridor.models import (
    Position, Game, VerticalWall, HorizontalWall, Player
)
import requests

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

url = 'http://localhost{}'


def get(route):
    r = requests.get(url.format(route))
    print r.text
    r.raise_for_status()
    return r.json()


def post(route, data={}, **kwargs):
    kwargs.update(data)
    r = requests.post(url.format(route), data=json.dumps(kwargs),
                      headers={'Content-Type': 'application/json'})
    print r.text
    r.raise_for_status()
    return r.json()


class TestQuoridorAPI(unittest.TestCase):

    def test_game_creation(self):
        print post('/game/new', description='test game')['id']

    def test_player_register(self):
        game = post('/game/new')['id']
        print post('/game/{}/register'.format(game),
                   name='User1', description='basically deep blue')

    def test_repeated_player_register(self):
        game = post('/game/new')['id']
        print post('/game/{}/register'.format(game), {'name': 'A'})
        with self.assertRaises(Exception):
            print post('/game/{}/register'.format(game), {'name': 'A'})

    def test_simple_game(self):
        game = post('/game/new')['id']
        post('/game/{}/register'.format(game), {'name': 'A'})
        post('/game/{}/register'.format(game), {'name': 'B'})
        post('/game/{}/start'.format(game))
        post('/game/{}/{}/move'.format(game, 'A'), {'direction': 'DOWN'})
        get('/game/{}/state'.format(game))
        with self.assertRaises(Exception):
            post('/game/{}/{}/move'.format(game, 'A'), {'direction': 'DOWN'})
        for i in range(7):
            post('/game/{}/{}/move'.format(game, 'B'), {'direction': 'UP'})
            post('/game/{}/{}/move'.format(game, 'A'), {'direction': 'DOWN'})
        print get('/game/{}/ascii'.format(game))['msg']
