use std::sync::LazyLock;
use rand::prelude::StdRng;
use rand::{RngExt, SeedableRng};
use crate::pieces::{Colour, Piece};
use crate::position::Position;
use crate::squares::Square;

const HASH_SEED: u64 = 36531637032464968u64;

static  HASH_KEYS: LazyLock<[u64; 769]> = LazyLock::new(init_values);

fn init_values() -> [u64; 769] {
    let mut values = [0; 769];
    let mut rng = StdRng::seed_from_u64(HASH_SEED);
    for x in 0..769 {
        let n: u64 = rng.random_range(0..u64::MAX);
        values[x] = n;
    }
    values
}

pub fn piece_key(piece: Piece, colour: Colour, sq: Square) -> u64 {
    let index = (colour as usize * 6 * 64) + (piece as usize * 64) + sq as usize;
    HASH_KEYS[index]
}

pub fn calculate_hash(pos: &Position) -> u64 {
    let mut hash = 0u64;

    for side in [Colour::White, Colour::Black] {
        for piece in Piece::ALL {
            for sq in pos.get_piece(side, *piece) {
                let index = (side as usize * 6 * 64) + (*piece as usize * 64) + sq as usize;
                hash ^= HASH_KEYS[index];
            }
        }
    }

    hash
}