// Copyright (c) 2015-2016 Joshua S. Miller
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Quoridor board data structures and logic

use player::Player;
use board::{Direction, Wall};
use quoridor::Game;
use errors::{QuoridorError, QuoridorResult};

pub enum Turn {
    Move(Direction),
    PlaceWall(Wall),
    Jump(Direction),
}

impl Turn {
    /// Take a turn
    pub fn take(&mut self, game: &mut Game, player: Player) -> QuoridorResult<()> {
        match *self {
            Turn::PlaceWall(wall) => {
                let _ = game.add_wall(&wall);
            },
            Turn::Move(direction) => {
                let _ = game.move_player(player.name, direction);
            },
            Turn::Jump(direction) => {
                let _ = game.move_player(player.name, direction);
            },
        };
        Ok(())
    }

    /// Undo a turn
    pub fn undo(&mut self, game: &mut Game, player: Player) -> QuoridorResult<()> {
        match *self {
            Turn::PlaceWall(wall) => {
                game.walls.remove(&wall);
            },
            Turn::Move(direction) => {
                let _ = game.move_player(player.name, direction.reversed());
            },
            Turn::Jump(direction) => {
                let _ = try!(game.is_valid_jump(player.p, player.p.shift(direction)));
            },
        };
        Ok(())
    }
}
