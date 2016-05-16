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

//! Quoridor errors

use std::fmt;

pub type QuoridorResult<T> = Result<T, QuoridorError>;

#[derive(Debug)]
pub enum QuoridorError {
    TurnError(String),
    PlayerNotFound,
    RegistrationError(String),
    InvalidJump(String),
    InvalidMove(String),
    InvalidWall(String),
}

impl fmt::Display for QuoridorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QuoridorError::TurnError(ref s) => write!(f, "Invalid turn: {}", s),
            QuoridorError::PlayerNotFound => write!(f, "Player not found"),
            QuoridorError::InvalidJump(ref s) => write!(f, "Invalid jump: {}", s),
            QuoridorError::InvalidMove(ref s) => write!(f, "Invalid move: {}", s),
            QuoridorError::InvalidWall(ref s) => write!(f, "Invalid wall: {}", s),
            QuoridorError::RegistrationError(ref s) => write!(f, "Registration error: {}", s),
        }
    }
}
