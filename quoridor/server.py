from flask import Flask, request, jsonify, abort,\
    Response, send_from_directory
from werkzeug.exceptions import default_exceptions
from werkzeug.exceptions import HTTPException
from functools import wraps
from models import Game, Player, VerticalWall, HorizontalWall, Position
import json
import os


app = Flask(__name__, static_url_path='')
games = {}


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
        except Exception as msg:
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
    games[game.game_id] = game
    return jsonify({
        'msg': 'Registered new game!',
        'id': game.game_id
    }), 201


@try_action
@app.route('/game/<game_id>/register', methods=['POST'])
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
    game = games[game_id]
    descrip = request.args.get('description', None)
    p = Player(name, descrip)
    game.register(p)
    return jsonify({'msg': 'Registered {}!'.format(p.name)}), 201


@try_action
@app.route('/game/<game_id>/start', methods=['POST'])
def start(game_id):
    """

    start the game. nothing more, nothing less.

    """
    games[game_id].start()
    return jsonify(msg='GAME {} HAS STARTED!!!!'.format(game_id))


@try_action
@app.route('/game/<game_id>/state', methods=['GET'])
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


@try_action
@app.route('/game/<game_id>/ascii', methods=['GET'])
def get_board(game_id):
    """Get ascii rep of the game.

    **Example response:**:

    .. code-block:: http

       HTTP/1.1 200 OK
       Content-Type: text/plain

    """
    return Response(str(games[game_id].board), mimetype='text/plain')


@try_action
@app.route('/game/<game_id>/<player>/move', methods=['POST'])
def move(game_id, player):
    """
    """
    direction, jump = request.args['direction'], request.args.get('jump', None)
    games[game_id].get_player(player).move(direction, jump)
    return jsonify(msg='Player {} moved {}'.format(player, direction))


if __name__ == '__main__':
    app.run(port=80, debug=True)
