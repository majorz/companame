#![allow(dead_code)]

#[macro_use] extern crate clap;

use std::str::from_utf8_unchecked;
use std::collections::{BTreeMap, BTreeSet};

use clap::{Arg, App};

struct Config {
    count: usize,
    delta: usize,
    start: Vec<u8>,
    end: Vec<u8>,
}

const CHARS: &[u8] = b"abcdefghiklmnoprstuvxyz";
const CONSONANTS: &[u8] = b"bcdfghklmnprstvxz";
const VOWELS: &[u8] = b"aeiou";

const LIKEY: [&[u8]; 74] = [
    b"mint" as &[u8],
    b"void",
    b"nyx",
    b"git",
    b"dell",
    b"pixel",
    b"flip",
    b"lake",
    b"disk",
    b"blade",
    b"flex",
    b"max",
    b"snap",
    b"react",
    b"volt",
    b"bayer",
    b"beyer",
    b"nikon",
    b"scope",
    b"apex",
    b"dice",
    b"nike",
    b"pike",
    b"spike",
    b"red",
    b"clip",
    b"base",
    b"bite",
    b"brick",
    b"cell",
    b"chase",
    b"chip",
    b"cold",
    b"deck",
    b"dick",
    b"fact",
    b"farm",
    b"file",
    b"frame",
    b"mark",
    b"mix",
    b"park",
    b"phase",
    b"race",
    b"act",
    b"arm",
    b"barn",
    b"void",
    b"back",
    b"bed",
    b"black",
    b"block",
    b"cap",
    b"lap",
    b"case",
    b"cast",
    b"check",
    b"class",
    b"cost",
    b"cross",
    b"cut",
    b"hut",
    b"kick",
    b"lock",
    b"nice",
    b"rice",
    b"next",
    b"pack",
    b"pick",
    b"sake",
    b"shell",
    b"stick",
    b"slick",
    b"vote",
];

type Map = BTreeMap<u8, BTreeSet<u8>>;


fn main() {
    let (start, end, middle) = gen_tables();

    let config = get_config();

    for word in LIKEY.iter() {
        if !valid(&word, &start, &end, &middle) {
            let word = unsafe { from_utf8_unchecked(&word) };

            panic!("!likey {}", word);
        }
    }

    let mut generated = 0;

    for chars in chars_product(CHARS, config.delta) {
        let mut word: Vec<u8> = Vec::with_capacity(config.count);
        word.extend(&config.start);
        word.extend(chars);
        word.extend(&config.end);

        if !valid(&word, &start, &end, &middle) {
            continue;
        }

        let word = unsafe { from_utf8_unchecked(&word as &[u8]) };

        println!("{}", word);

        generated += 1;
    }

    println!("Generated: {}", generated);
}

fn init_map() -> Map {
    let mut map = BTreeMap::new();

    for ch in CHARS {
        map.insert(*ch, BTreeSet::new());
    }

    map
}

fn gen_tables() -> (Map, Map, Map) {
    let mut start = init_map();
    let mut end = init_map();
    let mut middle = init_map();

    for word in LIKEY.iter() {
        let len = word.len();
        
        start.get_mut(&(word[0])).unwrap().insert(word[1]);

        end.get_mut(&(word[len-2])).unwrap().insert(word[len-1]);

        for i in 0..len - 3 {
            middle.get_mut(&(word[i+1])).unwrap().insert(word[i+2]);
        }
    }

    (start, end, middle)
}

fn print_table(map: Map) {
    for ch in CHARS {
        print!("        b'{}' => b\"", *ch as char);

        let mut vec: Vec<_> = map.get(&ch).unwrap().into_iter().collect();
        vec.sort();
        for inner in &vec {
            print!("{}", (**inner) as char);
        }

        println!("\",");
    }
}

fn triple(first: u8, second: u8, third: u8) -> bool {
    let triad = [first, second, third];

    let allowed = [
        b"rch" as &[u8],
        b"str" as &[u8],
    ];

    if allowed.contains(&(&triad as &[u8])) {
        return true;
    }

    if CONSONANTS.contains(&first) && CONSONANTS.contains(&second) && CONSONANTS.contains(&third) {
        return false;
    }

    if VOWELS.contains(&first) && VOWELS.contains(&second) && VOWELS.contains(&third) {
        return false;
    }

    true
}

fn valid(word: &[u8], start: &Map, end: &Map, middle: &Map) -> bool {
    let count = word.len();

    for i in 0..count - 2 {
        if !triple(word[i], word[i+1], word[i+2]) {
            return false;
        }
    }

    if !start.get(&word[0]).unwrap().contains(&word[1]) {
        return false;
    }

    if !end.get(&word[count-2]).unwrap().contains(&word[count-1]) {
        return false;
    }

    for i in 0..count - 3 {
        if !middle.get(&word[i+1]).unwrap().contains(&word[i+2]) {
            return false;
        }
    }

    true
}

fn chars_product(chars: &[u8], repeat: usize) -> CharsProduct {
    CharsProduct {
        started: false,
        chars: chars,
        repeat: repeat,
        indices: vec![0; repeat],
    }
}

struct CharsProduct<'a> {
    started: bool,
    chars: &'a [u8],
    repeat: usize,
    indices: Vec<usize>,
}

impl<'a> Iterator for CharsProduct<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        if !self.started {
            self.started = true;
            return Some(vec![self.chars[0]; self.repeat]);
        }
        
        for i in 0..self.repeat {
            if self.indices[i] == self.chars.len() - 1 {
                if i == self.repeat - 1 {
                    return None;
                }
                self.indices[i] = 0;
            } else {
                self.indices[i] += 1;

                let mut product = Vec::with_capacity(self.repeat);

                for j in 0..self.repeat {
                    product.push(self.chars[self.indices[self.repeat - j - 1]]);
                }

                return Some(product);
            }
        }

        None
    }
}

fn get_config() -> Config {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("count")
                .short("c")
                .default_value("3")
                .help("Character count")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("start")
                .short("s")
                .default_value("")
                .help("Start sequence")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("end")
                .short("e")
                .default_value("")
                .help("End sequence")
                .takes_value(true),
        )
        .get_matches();

    let count = value_t!(matches, "count", usize).unwrap();
    let start = value_t!(matches, "start", String).unwrap().into_bytes();
    let end = value_t!(matches, "end", String).unwrap().into_bytes();

    let delta = if let Some(delta) = count.checked_sub(start.len() + end.len()) {
        delta
    } else {
        panic!("Character count exceeds start and end sequence sum");
    };

    Config {
        count,
        delta,
        start,
        end,
    }
}
