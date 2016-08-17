use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Row <'a> {
    id: u32,
    name: &'a str,
    _1 : u32,
    _2 : u32,
    _3 : u32,
    _4 : u32,
    _5 : u32,
    _6 : u32,
    _7 : u32,
    _8 : u32,
    _9 : u32,
    _10 : u32,
    _11 : u32,
    _12 : u32,
}

fn next_u32<'a>(split : &mut std::str::Split<'a, &str>) -> u32 {
    split
    .next()
    .expect("key must be given")
    .parse::<u32>()
    .expect("key must be an int")
}

fn main() {

    let stdin = io::stdin();

    for line in stdin.lock().lines() {

        let line : String = line.expect("failed to read line");
        let mut split = line.split(",");

        let id = split
            .next()
            .expect("key must be given")
            .parse::<u32>()
            .expect("key must be an int");

        let name = split
            .next()
            .expect("value must be given");

        let row = Row {
            id: id,
            name: name,
            _1 : next_u32(&mut split),
            _2 : next_u32(&mut split),
            _3 : next_u32(&mut split),
            _4 : next_u32(&mut split),
            _5 : next_u32(&mut split),
            _6 : next_u32(&mut split),
            _7 : next_u32(&mut split),
            _8 : next_u32(&mut split),
            _9 : next_u32(&mut split),
            _10 : next_u32(&mut split),
            _11 : next_u32(&mut split),
            _12 : next_u32(&mut split),
        };

    }


}
