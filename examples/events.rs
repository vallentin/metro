use metro::Event;

fn main() {
    let events = [
        Event::station(0, "Station 1"),
        Event::station(0, "Station 2"),
        Event::station(0, "Station 3"),
        Event::SplitTrack(0, 1),
        Event::station(1, "Station 4"),
        Event::SplitTrack(1, 2),
        Event::station(1, "Station 5"),
        Event::station(2, "Station 6"),
        Event::station(0, "Station 7"),
        Event::station(1, "Station 8"),
        Event::station(2, "Station 9"),
        Event::SplitTrack(2, 3),
        Event::SplitTrack(3, 4),
        Event::station(5, "Station 10 (Detached)"),
        Event::JoinTrack(4, 0),
        Event::station(3, "Station 11"),
        Event::StopTrack(1),
        Event::station(0, "Station 12"),
        Event::station(2, "Station 13"),
        Event::station(3, "Station 14"),
        Event::JoinTrack(3, 0),
        Event::station(2, "Station 15"),
        Event::StopTrack(2),
        Event::station(0, "Station 16"),
    ];

    let string = metro::to_string(&events).unwrap();

    println!("{}", string);
}
