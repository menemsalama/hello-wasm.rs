#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;
extern crate instant;
extern crate sha1;

#[cfg(target_arch = "wasm32")]
use stdweb::js_export;

use instant::Instant;
use sha1::Sha1;
use std::env;

mod console;

// MAXZEROES safeguard to stay away of big computations
const MAXZEROES: u8 = 5;
// MINZEROES default for parsing errors
const MINZEROES: u8 = 1;

macro_rules! message {
    ($nonce:expr) => {
        format!("The winner is Wasm, Nonce is {}", $nonce);
    };
}

#[cfg_attr(target_arch = "wasm32", js_export)]
fn hash(string: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(string.as_bytes());
    hasher.digest().to_string()
}

#[cfg_attr(target_arch = "wasm32", js_export)]
fn mine(zeroes: u8) -> String {
    let start = Instant::now();

    let mut nonce = 0;
    let times = if zeroes > MAXZEROES {
        MAXZEROES
    } else if zeroes < MINZEROES {
        MINZEROES
    } else {
        zeroes
    };
    let zeroes = "0".repeat(times as usize);

    loop {
        let msg = message!(&nonce);
        let hashed_msg = hash(&msg);

        nonce += 1;
        if hashed_msg.starts_with(&zeroes) {
            log!(
                "Rust duration in milliseconds {}",
                start.elapsed().as_millis()
            );
            return hashed_msg;
        }
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    stdweb::initialize();

    let args: Vec<String> = env::args().collect();
    let n: u8 = if args.len() > MINZEROES as usize {
        args[1].parse().unwrap_or_default()
    } else {
        0
    };

    if n > 0 {
        log!("mining for {} zeroes", n);
        let results = mine(n);
        log!(results);
    }
}
