//! Quoridor game server
//!
//! author: Joshua Miller
//! email: jsmiller@uchicago.edu

use router::Router;
use iron::status;
use rustc_serialize::json::ToJson;
use rustc_serialize::json;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Condvar;
use std::sync::RwLock;
use std::io::Read;
use iron::prelude::*;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use quoridor::Game;
use quoridor::Wall;
use quoridor::_p;
use quoridor::GAME_OVER;
use quoridor::GAME_NOT_STARTED;

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct PlayerRegistrationRequest {
    name: String,
    key: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct PlayerMoveToRequest {
    name: String,
    key: String,
    position: [i32; 2],
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct PlayerMoveRequest {
    name: String,
    key: String,
    direction: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
struct WallRequest {
    name: String,
    key: String,
    p1: [i32; 2],
    p2: [i32; 2],
}

macro_rules! register_post_route {
    ($router: expr, $route: expr, $handler: expr, $game: expr) => {
        {
            let temp = $game.clone();
            $router.post($route, move |r: &mut Request|
                        $handler(r, &mut temp.write().unwrap()));
        }
    };
}

macro_rules! register_turn_route {
    ($router: expr, $cond: expr, $route: expr, $handler: expr, $game: expr) => {
        {
            let game_clone = $game.clone();
            let cond_clone = $cond.clone();
            $router.post($route, move |r: &mut Request| {
                let ret = $handler(r, &mut game_clone.write().unwrap());

                // If turn successful, notify all waiting for the next turn
                if ret.as_ref().unwrap().status.unwrap() == status::Ok {
                    let &(ref lock, ref cvar) = &*cond_clone;
                    lock.lock().ok();
                    cvar.notify_all();
                }
                ret
            });
        }
    };
}

macro_rules! register_get_route {
    ($router: expr, $route: expr, $handler: expr, $game: expr) => {
        {
            let temp = $game.clone();
            $router.get($route, move |r: &mut Request|
                        $handler(r, &mut temp.read().unwrap()));

        }
    };
}

// ======================================================================
// Macros for parsing JSON bodies and calling mutation functions

macro_rules! parse_payload {
    ($request: expr) => {
        {
            let mut payload = String::new();
            $request.body.read_to_string(&mut payload).unwrap();
            let temp = json::decode(&payload);
            if temp.is_err() {
                return Ok(Response::with(
                    (status::BadRequest, format!(
                        "Unable to processrequest: {:?}", temp.err()))))
            }
            temp.unwrap()
        }
    };
}

macro_rules! try_call {
    ($game: expr, $call: expr) => {
        {
            match $call {
                Ok(_) => Ok(Response::with(
                    (status::Ok, $game.to_json().to_string()))),
                Err(e) => Ok(Response::with((status::BadRequest, e)))
            }
        }
    };
}


macro_rules! take_turn {
    ($game: expr, $name: expr, $key:expr, $call: expr) => {
        {
            check_player!($game, $name, $key);
            let ret = try_call!($game, $call);
            if ret.as_ref().unwrap().status.unwrap() == status::Ok {
                $game.next_turn();
            }
            ret
        }
    };
}

macro_rules! check_player {
    ($game: expr, $name: expr, $key: expr) => {
        {
            if !$game.players.contains_key(&$name) {
                return Ok(Response::with(
                    (status::BadRequest, "Player not found")))
            }
            if $game.players[&$name].key != $key {
                return Ok(Response::with(
                    (status::BadRequest, "Unauthorized move.")))
            }
            if $game.turn == GAME_NOT_STARTED {
                return Ok(Response::with(
                    (status::BadRequest, "Waiting on other players.")))
            }
            if $game.turn == GAME_OVER {
                return Ok(Response::with(
                    (status::BadRequest, "The game is over!")))
            }
            if $game.players[&$name].id as i32 != $game.turn {
                return Ok(Response::with(
                    (status::BadRequest, "Not your turn.")))
            }
        }
    };
}

/// *********************************************************************
/// Handlers for API endpoints
/// ********************************************************************

fn get_status(request: &mut Request, game: &Game) -> IronResult<Response> {
    println!("{:?}", request);
    let payload = game.to_json().to_string();
    Ok(Response::with((status::Ok, payload)))
}


fn get_ascii(request: &mut Request, game: &Game) -> IronResult<Response> {
    println!("{:?}", request);
    let payload = game.to_string();
    Ok(Response::with((status::Ok, payload)))
}


fn register_player(request: &mut Request, game: &mut Game) -> IronResult<Response> {
    println!("{:?}", request);
    let data: PlayerRegistrationRequest = parse_payload!(request);
    try_call!(game, game.add_player(data.name, data.key))
}

fn move_player_to(request: &mut Request, game: &mut Game) -> IronResult<Response> {
    println!("{:?}", request);
    let data: PlayerMoveToRequest = parse_payload!(request);
    take_turn!(game,
               data.name,
               data.key,
               game.move_player_to(data.name, _p(data.position[0], data.position[1])))
}

fn move_player(request: &mut Request, game: &mut Game) -> IronResult<Response> {
    println!("{:?}", request);
    let data: PlayerMoveRequest = parse_payload!(request);
    take_turn!(game,
               data.name,
               data.key,
               game.move_player(data.name, data.direction))
}

fn place_wall(request: &mut Request, game: &mut Game) -> IronResult<Response> {
    println!("{:?}", request);
    let data: WallRequest = parse_payload!(request);
    let a = (data.p1[0], data.p1[1]);
    let b = (data.p2[0], data.p2[1]);
    let wall = Wall::from_tuples(a, b);
    match wall {
        Ok(w) => take_turn!(game, data.name, data.key, game.add_wall(&w)),
        Err(e) => Ok(Response::with((status::BadRequest, &*e)))
    }
}

fn wait_for_activity(request: &mut Request,
                     lock: &Mutex<bool>,
                     cvar: &Condvar)
                     -> IronResult<Response> {
    println!("{:?}", request);
    let taken = lock.lock().unwrap();
    let _ = cvar.wait(taken).unwrap();
    Ok(Response::with((status::Ok, "A turn was taken")))
}

pub fn listen(host: String, _game: Game) {
    let cond = Arc::new((Mutex::new(false), Condvar::new()));
    let game = Arc::new(RwLock::new(_game));
    let mut router = Router::new();

    // POST
    register_turn_route!(router, cond, "/register_player", register_player, game);
    register_turn_route!(router, cond, "/move_player_to", move_player_to, game);
    register_turn_route!(router, cond, "/move_player", move_player, game);
    register_turn_route!(router, cond, "/place_wall", place_wall, game);

    // GET
    register_get_route!(router, "/state", get_status, game);
    register_get_route!(router, "/ascii", get_ascii, game);

    {
        // Special case to wait for next turn
        let cond_clone = cond.clone();
        router.get("/wait_for_activity", move |r: &mut Request| {
            let &(ref lock, ref cvar) = &*cond_clone;
            wait_for_activity(r, lock, cvar)
        });
    }

    let mut mount = Mount::new();
    mount.mount("/api", router)
         .mount("/game", Static::new(Path::new("static")));

    println!("Listening on {} ...", host);
    Iron::new(mount).http(&*host).unwrap();

}
