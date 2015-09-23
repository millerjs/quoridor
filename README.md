# quoridor
[![Build Status](https://api.travis-ci.org/millerjs/quoridor.svg?branch=master)](https://api.travis-ci.org/millerjs/quoridor.svg)

A [quoridor](https://en.wikipedia.org/wiki/Quoridor) game server implementation in [Rust](https://www.rust-lang.org/) using [Iron](https://github.com/iron/iron).

### Requirements

- [Rust 1.3.0](https://www.rust-lang.org/install.html) - A systems programming language that runs blazingly fast, prevents almost all crashes, and eliminates data races
- [Cargo](https://crates.io/) - The Rust package manager


### Build

To build and run, install [Rust 1.3.0](https://www.rust-lang.org/install.html) and [Cargo](https://crates.io/). (Rust dependencies will be installed automatically.)

#### Run tests

```rust
cargo test
```

#### Build and run server on port 9999

```rust
cargo run 0:9999
```

### Examples using curl


#### Register two players

```
curl -XPOST localhost:9999/api/register_player -d '{
     "name": "Player 1",
     "key": "abcd"
}'
curl -XPOST localhost:9999/api/register_player -d '{
     "name": "Player 2",
     "key": "efgh"
}'
```

#### Get ascii representation of the board

```
curl -XGET localhost:9999/api/ascii
```

**Output:**

```
    -1   0   1   2   3   4   5   6   7   8   9
   + - + - + - + - + - + - + - + - + - + - + - +
-1 |   |   |   |   |   |   |   |   |   |   |   |
   + - +   +   +   +   +   +   +   +   +   + - +
 0 |                     0                     |
   + - +   +   +   +   +   +   +   +   +   + - +
 1 |                                           |
   + - +   +   +   +   +   +   +   +   +   + - +
 2 |                                           |
   + - +   +   +   +   +   +   +   +   +   + - +
 3 |                                           |
   + - +   +   +   +   +   +   +   +   +   + - +
 4 |                                           |
   + - +   +   +   +   +   +   +   +   +   + - +
 5 |                                           |
   + - +   +   +   +   +   +   +   +   +   + - +
 6 |                                           |
   + - +   +   +   +   +   +   +   +   +   + - +
 7 |                                           |
   + - +   +   +   +   +   +   +   +   +   + - +
 8 |                     1                     |
   + - +   +   +   +   +   +   +   +   +   + - +
 9 |   |   |   |   |   |   |   |   |   |   |   |
   + - + - + - + - + - + - + - + - + - + - + - +
```

### Get board state

```
curl localhost:9999/api/state
```

**Output:**

```
{
    "players": [
        {
            "id": 0,
            "name": "Player 1",
            "position": [
                4,
                1
            ],
            "walls": 10
        },
        {
            "id": 1,
            "name": "Player 2",
            "position": [
                4,
                7
            ],
            "walls": 10
        }
    ],
    "size": 9,
    "turn": 0,
    "walls": []
}
```

#### Move player by direction

```
curl -XPOST localhost:9999/api/move_player -d '{
     "name": "Player 1",
     "key": "abcd",
     "direction": "down",
}'
```

#### Move player to position

Pro tip: This is how you perform jumps.

```
curl -XPOST localhost:9999/api/move_player_to -d '{
     "name": "Player 2",
     "key": "efgh",
     "position": [4, 7],
}'
```

### Wait for another player to take a turn

In order to know as soon as it's your turn, you can make a blocking
call on the server!

```
curl -XGET localhost:9999/api/wait_for_activity
```
