from flask import Flask, request, jsonify, abort,\
    Response, send_from_directory
from werkzeug.exceptions import default_exceptions
from werkzeug.exceptions import HTTPException
from functools import wraps
from models import Game, Player, VerticalWall, HorizontalWall, Position
import json
import os
import logging
import traceback


app = Flask(__name__, static_url_path='')
games = {}
log = logging.getLogger('server')


def get_game(game_id):
    assert game_id in games,\
        'Game {} not found'.format(game_id)
    return games[game_id]


def save_game(game):
    games[game.game_id] = game


def make_json_error(ex):
    response = jsonify(msg=str(ex.description))
    response.status_code = (
        ex.code if isinstance(ex, HTTPException) else 500)
    return response

for code in default_exceptions.iterkeys():
    app.error_handler_spec[None][code] = make_json_error


def try_action(func):
    @wraps(func)
    def action(*args, **kwargs):
        try:
            return func(*args, **kwargs)
        except AssertionError as msg:
            log.error('{}: {}'.format(msg.__class__.__name__, msg))
            abort(400, description=msg)
    return action


@app.route('/game', methods=['POST'])
def new_game():
    """Creates a new game.

    **Example request**:

    .. code-block:: http

       POST /game HTTP/1.1
       Content-Type: application/json

    .. code-block:: javascript

       {
         'description': 'a boring game'
       }

    **Example response**:

    .. code-block:: http

       HTTP/1.1 201 CREATED
       Content-Type: application/json

    .. code-block:: javascript

       {
         'msg': 'Registered new game!',
         'id': 'game1',
       }

    :reqheader Content-Type: nothin but json
    :resheader Content-Type: Only json
    :<json str description: house rules?
    :>json str id: The id of the new game
    :statuscode 201: User created
    :statuscode 400: Bad request

    """
    data = request.get_json()
    descrip = data.get('description', '')
    game = Game(description=descrip)
    save_game(game)
    return jsonify({
        'msg': 'Registered new game!',
        'id': game.game_id
    }), 201


@app.route('/game/<game_id>/register', methods=['POST'])
@try_action
def register(game_id):
    """Register a player to game `game_id`.

    **Example request**:

    .. code-block:: http

       POST /game/game1/register HTTP/1.1

    .. code-block:: javascript

       {
         "name": "Mario",
         "description": "It's a me, a-Mario!"
       }

    **Example response**:

    .. code-block:: http

       HTTP/1.1 201 CREATED
       Vary: Accept
       Content-Type: text/javascript

    .. code-block:: javascript

       {
         'msg': 'Registered Mario!'
       }

    :<json str name: The name of the player you are registering
    :<json str description: Your custom message, status, whatever
    :reqheader Accept: requires json set in :mailheader:`Accept` header
    :resheader Content-Type: returns json
    :statuscode 201: User created
    :statuscode 400: Bad request
    """

    data = request.get_json()
    descrip = data.get('description', '')
    name = data['name']
    game = get_game(game_id)
    descrip = request.args.get('description', None)
    p = Player(name, descrip)
    game.register(p)
    return jsonify({
        'msg': 'Registered {}!'.format(p.name),
        'player_id': p.player_id,
    }), 201


@app.route('/game/<game_id>/start', methods=['POST'])
@try_action
def start(game_id):
    """

    start the game. nothing more, nothing less.

    """
    game = get_game(game_id)
    game.start()
    return jsonify(msg='GAME {} HAS STARTED!!!!'.format(game_id))


@app.route('/game/<game_id>/state', methods=['GET'])
@try_action
def state(game_id):
    """Get the game state

    **Example response:**:

    .. code-block:: http

       HTTP/1.1 200 OK
       Content-Type: text/plain

    .. code-block:: javascript

       {
         "players": {
           "A": {
             "direction": "DOWN",
             "walls": 10,
             "x": 5,
             "y": 1
           },
           "B": {
             "direction": "UP",
             "walls": 10,
             "x": 5,
             "y": 9
           }
         },
         "started": true
       }

    """
    return jsonify(games[game_id].to_json())


@app.route('/game/<game_id>/ascii', methods=['GET'])
@try_action
def get_board(game_id):
    """Get ascii rep of the game.

    **Example response:**:

    .. code-block:: http

       HTTP/1.1 200 OK
       Content-Type: text/plain

    """
    game = get_game(game_id)
    return Response(str(game.board), mimetype='text/plain')


@app.route('/game/<game_id>/<player_id>/move/<direction>', methods=['POST'])
@app.route('/game/<game_id>/<player_id>/move/<direction>/<jump>', methods=['POST'])
@try_action
def move(game_id, player_id, direction, jump=None):
    """
    """
    direction = direction.upper()
    jump = None if not jump else jump.upper()
    assert direction in ['RIGHT', 'UP', 'LEFT', 'DOWN'],\
        'Invalid move direction {}'.format(direction)
    game = get_game(game_id)
    player = game.get_player(player_id)
    old_pos = player.pos
    player.move(direction, jump)
    return jsonify({
        'msg': '{} moved {}'.format(player.name, direction),
        'old_position': {
            'x': old_pos.x,
            'y': old_pos.y,
        },
        'new_position': {
            'x': player.pos.x,
            'y': player.pos.y,
        }
    })


@app.route('/game/<game_id>/<player_id>/wall/<direction>/<int:x>/<int:y>',
           methods=['POST'])
@try_action
def place_wall(game_id, player_id, direction, x, y):
    """
    """
    direction = direction.upper()
    assert direction in ['VERTICAL', 'HORIZONTAL'],\
        'Invalid wall orientation {}, try, vertical or horizontal'.format(
            direction)
    game = get_game(game_id)
    player = game.get_player(player_id)
    wall = {
        'VERTICAL': VerticalWall,
        'HORIZONTAL': HorizontalWall,
    }[direction](Position(x, y))
    player.place_wall(wall)
    return jsonify({
        'msg': '{} placed a wall {}'.format(player.name, direction),
    })

if __name__ == '__main__':
    app.run(port=8000, debug=True)
