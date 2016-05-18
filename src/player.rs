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

//! Defines a player in a game of Quoridor

use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::collections::BTreeMap;
use board::Point;
use constants::N;

#[derive(Debug)]
pub struct Player {
    pub p: Point,
    pub key: String,
    pub id: u8,
    pub walls: u8,
    pub name: String,
}


impl Player {
    pub fn has_won(&self) -> bool {
        (self.id != 1 && self.p.y < 0)
            || (self.id != 2 && self.p.x < 0)
            || (self.id != 0 && self.p.y >= N)
            || (self.id != 3 && self.p.y >= N)
    }

    pub fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("position".to_string(), vec![self.p.x, self.p.y].to_json());
        d.insert("id".to_string(), self.id.to_json());
        d.insert("walls".to_string(), self.walls.to_json());
        d.insert("name".to_string(), self.name.to_json());
        Json::Object(d)
    }
}
