# metro

[![Build Status](https://travis-ci.org/vallentin/metro.svg?branch=master)](https://travis-ci.org/vallentin/metro)
[![Latest Version](https://img.shields.io/crates/v/metro.svg)](https://crates.io/crates/metro)
[![Docs](https://docs.rs/metro/badge.svg)](https://docs.rs/metro)
[![License](https://img.shields.io/github/license/vallentin/metro.svg)](https://github.com/vallentin/metro)

Metro is a crate for rendering graphs similar to `git log --graph`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
metro = "0.1"
```

## Example

```rust
use metro::Event::*;
let events = [
    Station(0, "Station 1"),
    Station(0, "Station 2"),
    NoEvent,
    Station(0, "Station 3"),
    SplitTrack(0, 1),
    Station(1, "Station 4"),
    SplitTrack(1, 2),
    Station(1, "Station 5"),
    Station(2, "Station 6"),
    NoEvent,
    Station(0, "Station 7"),
    Station(1, "Station 8"),
    Station(2, "Station 9"),
    SplitTrack(2, 3),
    SplitTrack(3, 4),
    Station(5, "Station 10"),
    JoinTrack(4, 0),
    Station(3, "Station 11"),
    StopTrack(1),
    NoEvent,
    Station(0, "Station 12"),
    Station(2, "Station 13"),
    Station(3, "Station 14"),
    JoinTrack(5, 0),
    JoinTrack(3, 0),
    Station(2, "Station 15"),
    StopTrack(2),
    NoEvent,
    JoinTrack(1, 0),
    Station(0, "Station 16"),
];

println!("{}", metro::to_string(&events).unwrap());
```

This will output the following:

```text
* Station 1
* Station 2
|
* Station 3
|\
| * Station 4
| |\
| * | Station 5
| | * Station 6
| | |
* | | Station 7
| * | Station 8
| | * Station 9
| | |\
| | | |\
| | | | | Station 10
| |_|_|/
|/| | |
| | | * Station 11
| " | |
|  / /
| | |
* | | Station 12
| * | Station 13
| | * Station 14
| |/
|/|
| * Station 15
| "
|
* Station 16
```
