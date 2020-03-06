use rand::seq::IteratorRandom;
use rand::thread_rng;

pub type SyncSecret = String;

const VALID_ID_CHARS: &str = "0123456789ABCDEFGHIJKLNMOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const NUMBER_OF_CHARS_IN_FULL_ID: usize = 64;

pub fn generate() -> SyncSecret {
    let mut result = String::new();

    for _ in 0..NUMBER_OF_CHARS_IN_FULL_ID {
        let random = VALID_ID_CHARS
            .chars()
            .choose(&mut thread_rng())
            .expect("Couldn't get random char");

        result.push(random);
    }

    result
}
