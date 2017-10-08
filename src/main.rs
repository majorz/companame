#![feature(test)]

#[macro_use] extern crate clap;
extern crate test;

use clap::{Arg, App};
use test::black_box;

struct Config {
    delta: usize,
    start: String,
    end: String,
}

const CHARS: &[u8] = b"abcdefghiklmnoprstuvxyz";
const CONSONANTS: &[u8] = b"bcdfghklmnprstvx";
const VOWELS: &[u8] = b"aeiou";

fn main() {
    let config = get_config();

    for chars in chars_product(CHARS, config.delta as usize) {
        black_box(chars);
    }
}

fn chars_product<'a>(chars: &'a [u8], repeat: usize) -> CharsProduct<'a> {
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
    let start = value_t!(matches, "start", String).unwrap();
    let end = value_t!(matches, "end", String).unwrap();

    let delta = if let Some(delta) = count.checked_sub(start.len() + end.len()) {
        delta
    } else {
        panic!("Character count exceeds start and end sequence sum");
    };

    Config {
        delta,
        start,
        end,
    }
}
