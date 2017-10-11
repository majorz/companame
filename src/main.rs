#[macro_use] extern crate clap;

use std::str::from_utf8_unchecked;
use std::collections::{BTreeMap, BTreeSet};

use clap::{Arg, App};

macro_rules! btreemap {
    // trailing comma case
    ($($key:expr => $value:expr,)+) => (btreemap!($($key => $value),+));
    
    ( $($key:expr => $value:expr),* ) => {
        {
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}

struct Config {
    count: usize,
    delta: usize,
    start: Vec<u8>,
    end: Vec<u8>,
}

const CHARS: &[u8] = b"abcdefghiklmnoprstuvxyz";
const CONSONANTS: &[u8] = b"bcdfghklmnprstvxz";
const VOWELS: &[u8] = b"aeiou";

const LIKEY: [&[u8]; 117] = [
    b"alpine" as &[u8],
    b"antix",
    b"arch",
    b"berry",
    b"endian",
    b"fedora",
    b"fermi",
    b"gecko",
    b"lite",
    b"mint",
    b"neon",
    b"salix",
    b"void",
    b"clio",
    b"pulse",
    b"focus",
    b"edge",
    b"ariel",
    b"atom",
    b"servo",
    b"xero",
    b"avon",
    b"chanel",
    b"etude",
    b"nyx",
    b"lush",
    b"lorac",
    b"melt",
    b"lime",
    b"dell",
    b"acer",
    b"pixel",
    b"flip",
    b"lake",
    b"disk",
    b"blade",
    b"factor",
    b"flex",
    b"max",
    b"pico",
    b"bliss",
    b"dylan",
    b"snap",
    b"react",
    b"sandy",
    b"trend",
    b"volt",
    b"accent",
    b"alert",
    b"gramm",
    b"signet",
    b"state",
    b"target",
    b"airbus",
    b"bayer",
    b"nikon",
    b"scope",
    b"vault",
    b"apex",
    b"dice",
    b"kinder",
    b"kite",
    b"moog",
    b"nike",
    b"penn",
    b"pike",
    b"lemm",
    b"shade",
    b"pray",
    b"red",
    b"break",
    b"clip",
    b"rush",
    b"bake",
    b"base",
    b"bite",
    b"brick",
    b"cake",
    b"camp",
    b"cell",
    b"chase",
    b"chip",
    b"cold",
    b"concept",
    b"contact",
    b"date",
    b"deck",
    b"draft",
    b"dress",
    b"drink",
    b"drive",
    b"earn",
    b"elite",
    b"enter",
    b"exist",
    b"extend",
    b"fact",
    b"farm",
    b"file",
    b"frame",
    b"fresh",
    b"grave",
    b"impact",
    b"mark",
    b"mix",
    b"park",
    b"phase",
    b"race",
    b"reach",
    b"recall",
    b"relax",
    b"result",
    b"stock",
    b"strike",
    b"think",
    b"trade",
    b"trip",
];

type Map = BTreeMap<u8, BTreeSet<u8>>;


fn main() {
    //gen_tables();

    let config = get_config();


    for word in LIKEY.iter() {
        if !valid(&word) {
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

        if !valid(&word) {
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

fn gen_tables() {
    let mut start_map = init_map();
    let mut end_map = init_map();
    let mut middle_map = init_map();

    for word in LIKEY.iter() {
        let len = word.len();
        
        start_map.get_mut(&(word[0])).unwrap().insert(word[1]);

        end_map.get_mut(&(word[len-2])).unwrap().insert(word[len-1]);

        for i in 0..len - 3 {
            middle_map.get_mut(&(word[i+1])).unwrap().insert(word[i+2]);
        }
    }

    print_table(end_map);
    
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

fn start_valid(first: u8, second: u8) -> bool {
    let good = btreemap!{
        b'a' => b"cilnprtv" as &[u8],
        b'b' => b"aeilr",
        b'c' => b"aehlo",
        b'd' => b"aeiry",
        b'e' => b"adlntx",
        b'f' => b"aeilor",
        b'g' => b"er",
        b'h' => b"",
        b'i' => b"m",
        b'k' => b"i",
        b'l' => b"aeiou",
        b'm' => b"aeio",
        b'n' => b"eiy",
        b'o' => b"",
        b'p' => b"aehiru",
        b'r' => b"aeu",
        b's' => b"acehint",
        b't' => b"ahr",
        b'u' => b"",
        b'v' => b"ao",
        b'x' => b"e",
        b'y' => b"",
        b'z' => b"",
    };

    good.get(&first).unwrap().contains(&second)
}


fn end_valid(first: u8, second: u8) -> bool {
    let good = btreemap!{
        b'a' => b"cknpxy" as &[u8],
        b'b' => b"",
        b'c' => b"ehkot",
        b'd' => b"ey",
        b'e' => b"dlrtx",
        b'f' => b"t",
        b'g' => b"e",
        b'h' => b"",
        b'i' => b"dopx",
        b'k' => b"eo",
        b'l' => b"delt",
        b'm' => b"eimp",
        b'n' => b"deknt",
        b'o' => b"gmnr",
        b'p' => b"et",
        b'r' => b"akmnoty",
        b's' => b"ehkst",
        b't' => b"e",
        b'u' => b"s",
        b'v' => b"eo",
        b'x' => b"",
        b'y' => b"x",
        b'z' => b"",
    };

    good.get(&first).unwrap().contains(&second)
}


fn middle(first: u8, second: u8) -> bool {
    let good = btreemap!{
        b'a' => b"cdfklmnrstuvy" as &[u8],
        b'b' => b"u",
        b'c' => b"acekotu",
        b'd' => b"egio",
        b'e' => b"acdlmnoprs",
        b'f' => b"",
        b'g' => b"en",
        b'h' => b"ai",
        b'i' => b"acegklmnrstvx",
        b'k' => b"o",
        b'l' => b"aeips",
        b'm' => b"p",
        b'n' => b"acdet",
        b'o' => b"cilnopr",
        b'p' => b"aei",
        b'r' => b"abcegimrv",
        b's' => b"u",
        b't' => b"aeioru",
        b'u' => b"dls",
        b'v' => b"o",
        b'x' => b"eit",
        b'y' => b"el",
        b'z' => b"",
    };

    good.get(&first).unwrap().contains(&second)
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

fn valid(word: &[u8]) -> bool {
    let count = word.len();

    for i in 0..count - 2 {
        if !triple(word[i], word[i+1], word[i+2]) {
            return false;
        }
    }

    if !start_valid(word[0], word[1]) {
        return false;
    }

    if !end_valid(word[count-2], word[count-1]) {
        return false;
    }

    for i in 0..count - 3 {
        if !middle(word[i+1], word[i+2]) {
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
                    product.push(self.chars[self.indices[j]]);
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
