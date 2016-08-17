#[macro_use]
extern crate nom;
use nom::{not_line_ending, line_ending, digit};
use nom::IResult;
use std::str;
use std::u32;
use std::str::FromStr;
use std::io;
use std::io::BufRead;
use std::io::Read;

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

named!(parse_line<&[u8],Row>,
  chain!(    // the parser takes a byte array as input, and returns an A struct
    id: map_res!(map_res!(digit, str::from_utf8), u32::from_str)  ~
    tag!(",") ~
    name: map_res!(take_until!(","), str::from_utf8) ~
    tag!(",") ~
    _1: map_res!(map_res!(digit, str::from_utf8), u32::from_str) ~
    tag!(",") ~
    _2: map_res!(map_res!(digit, str::from_utf8), u32::from_str) ~
    tag!(",") ~
    _3: map_res!(map_res!(digit, str::from_utf8), u32::from_str) ~
    tag!(",") ~
    _4: map_res!(map_res!(digit, str::from_utf8), u32::from_str) ~
    tag!(",") ~
    _5: map_res!(map_res!(digit, str::from_utf8), u32::from_str) ~
    tag!(",") ~
    _6: map_res!(map_res!(digit, str::from_utf8), u32::from_str) ~
    tag!(",") ~
    _7: map_res!(map_res!(digit, str::from_utf8), u32::from_str) ~
    tag!(",") ~
    _8: map_res!(map_res!(digit, str::from_utf8), u32::from_str) ~
    tag!(",") ~
    _9: map_res!(map_res!(digit, str::from_utf8), u32::from_str) ~
    tag!(",") ~
    _10: map_res!(map_res!(digit, str::from_utf8), u32::from_str) ~
    tag!(",") ~
    _11: map_res!(map_res!(digit, str::from_utf8), u32::from_str) ~
    tag!(",") ~
    _12: map_res!(map_res!(digit, str::from_utf8), u32::from_str),
    ||{Row{
        id: id,
        name: name,
        _1 : _1,
        _2 : _2,
        _3 : _3,
        _4 : _4,
        _5 : _5,
        _6 : _6,
        _7 : _7,
        _8 : _8,
        _9 : _9,
        _10 : _10,
        _11 : _11,
        _12 : _12,
    }} // the final closure will be able to use the variable defined previously
  )
);

fn main() {

    let mut a = std::io::stdin();
    let mut stdin = a.lock();

    loop {
        let consumed_bytes = consume_line(&mut stdin);
        stdin.consume(consumed_bytes);

        if consumed_bytes == 0 {
            break;
        }
    }
}

pub fn consume_line(stdin : &mut std::io::StdinLock) -> usize {

    let bytes = stdin.fill_buf().unwrap();

    if bytes.is_empty() {
        return 0;
    }

    match not_line_ending(&bytes) {
        IResult::Done(remaining_linies, line) => {
            //println!("{:?}", parse_line(&line));

            match line_ending(remaining_linies) {
                IResult::Done(_, endings) => {
                    line.len() + endings.len()
                },
                _ => line.len()
            }
        },
        _ => panic!("nope")
    }
}
