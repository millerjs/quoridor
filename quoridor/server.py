from flask import Flask, request, session, g, jsonify, abort
from werkzeug.exceptions import default_exceptions
from werkzeug.exceptions import HTTPException
from functools import wraps
from models import Game, Player, VerticalWall, HorizontalWall, Position
import json


app = Flask(__name__)
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


@app.route('/game/new', methods=['POST'])
def new_game():
    """Creates a new game.

    **Example request**:

    .. sourcecode:: http

       POST /game/new?description=a+boring+match HTTP/1.1
       Content-Type: application/json

    **Example response**:

    .. sourcecode:: http

       HTTP/1.1 201 CREATED
       Vary: Accept
       Content-Type: application/json

       {
         'msg': 'Registered new game!',
         'id': game1,
       }

    :query description: Game description
    :reqheader Accept: requires json set in :mailheader:`Accept` header
    :resheader Content-Type: returns json
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
    """Register a user to game `game_id`.

    **Example request**:

    .. sourcecode:: http

       POST /game/game1/register HTTP/1.1

       {
         "name": "Mario",
         "description": "It's a me, a-Mario!"
       }

    **Example response**:

    .. sourcecode:: http

       HTTP/1.1 201 CREATED
       Vary: Accept
       Content-Type: text/javascript

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
    """
    games[game_id].start()
    return jsonify(msg='GAME {} HAS STARTED!!!!'.format(game_id))


@try_action
@app.route('/game/<game_id>/state', methods=['GET', "POST"])
def state(game_id):
    """
    """
    return jsonify(games[game_id].to_json())


@try_action
@app.route('/game/<game_id>/ascii', methods=['GET'])
def get_board(game_id):
    """
    """
    return jsonify(msg=str(games[game_id].board))


@try_action
@app.route('/game/<game_id>/<player>/move', methods=['POST'])
def move(game_id, player):
    """
    """
    direction, jump = request.args['direction'], request.args.get('jump', None)
    games[game_id].get_player(player).move(direction, jump)
    return jsonify(msg='Player {} moved {}'.format(player, direction))


@app.route('/', methods=['GET'])
def show_help():
    return jsonify({
        'message': 'Welcome to the QUORIDOR ARENA!!!!!',
        'games': [a.game_id for a in games],
    })


if __name__ == '__main__':
    app.run(port=80, debug=True)
