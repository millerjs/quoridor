[![Build Status](https://api.travis-ci.org/millerjs/quoridor.svg?branch=master)](https://api.travis-ci.org/millerjs/quoridor.svg)

# quoridor
A [quoridor](https://en.wikipedia.org/wiki/Quoridor) game server implementation in Rust.

### Run tests

```rust
cargo test
```

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

#### Move player by direction

```
curl -XPOST localhost:9999/api/move_player -d '{
     "name": "Player 1",
     "key": "abcd",
     "direction": "down",
}'
```

#### Move player to position

```
curl -XPOST localhost:9999/api/move_player_to -d '{
     "name": "Player 2",
     "key": "efgh",
     "position": [4, 7],
}'
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

### Wait for another player to take a turn

In order to know as soon as it's your turn, you can make a blocking
call on the server!

```
curl -XGET localhost:9999/api/wait_for_activity
```