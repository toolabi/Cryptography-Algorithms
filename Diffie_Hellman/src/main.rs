
use rand::{rng, Rng};
fn main() {
    let p: u128 = 29;
    let g :u128= 5;

    // alice secret
    let x: u128 =4;
    let y :u128 = 8;

    let X = g.pow(x as u32) % p;
    let Y = g.pow(y as u32) % p;


    let alice_k = Y.pow(x as u32) %p;
    let bob_k = X.pow(y as u32) %p;


    assert_eq!(alice_k,bob_k);
}

