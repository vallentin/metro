# Changelog

## Version 0.2.0 (2020-??-??)

- Optimized `to_string` by using `from_utf8_unchecked` instead of `from_utf8` based on Metro always writing valid UTF-8 to output
- Removed `metro::Error`, replacing `Result<_, metro::Error>` with `std::io::Result<_>`
  - Affects `to_string`, `to_vec`, `to_writer`, `Metro::to_string`, `Metro::to_vec`, `Metro::to_writer`

## Version 0.1.1 (2020-02-19)

- Added `Metro`, and `Track`
- Fixed `Event::JoinTrack` producing extra whitespace at the end
- Fixed `Event::SplitTrack` rendering non-existing `from_track_id`

## Version 0.1.0 (2020-02-18)

- Added `Event`
- Added `to_string`, `to_vec`, and `to_writer`
