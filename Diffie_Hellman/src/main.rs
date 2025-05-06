
use rand::{rng, Rng};
fn main() {
    let p: u128 = 29;
    let g :u128= 5;

    // alice secret
    let x: u128 =4;
    // bob secret
    let y :u128 = 8;

     // Calculate public values 
    let X = g.pow(x as u32) % p;
    let Y = g.pow(y as u32) % p;


    // Calculate shared secrets
    let alice_k = Y.pow(x as u32) %p;
    let bob_k = X.pow(y as u32) %p;


    println!("Alice's secret: {}", x);
    println!("Bob's secret: {}", y);
    println!("Alice's shared key: {}", alice_k);
    println!("Bob's shared key: {}", bob_k);
    
    assert_eq!(alice_k,bob_k);
}

