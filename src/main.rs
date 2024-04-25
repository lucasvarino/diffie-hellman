use num_bigint::BigUint;
use rand::prelude::*;

// Defining the p choice (prime number)
const P: u64 = 4787;

// Calculate the public key with the given private key and g
fn public_key(private_key: u64, g: u64) -> u64 {
    let private_key_big = BigUint::from(private_key);
    let g_big = BigUint::from(g);
    let p_big = BigUint::from(P);

    let public_key_big = g_big.modpow(&private_key_big, &p_big);
    public_key_big.to_u64_digits()[0]
}

// Generate the private key
fn private_key() -> u64 {
    let mut rng = rand::thread_rng();
    let private_key: u64 = rng.gen_range(1..(P - 1));
    private_key
}

// Calculate the shared secret
fn shared_secret(private_key: u64, other_public_key: u64) -> u64 {
    let shared_secret = (other_public_key as f64).powi(private_key as i32) as u64 % P;
    shared_secret
}

// Verify if g is a primitive root of p
fn is_primitive_root(g: u64, p: u64) -> bool {
    let mut set = std::collections::HashSet::new();
    let mut val = 1;
    for _ in 0..p - 1 {
        val = val * g % p;
        if set.contains(&val) {
            return false;
        }
        set.insert(val);
    }
    true
}

// Generate a primitive root of p
fn primitive_root(p: u64) -> u64 {
    for g in 2..p {
        if is_primitive_root(g, p) {
            return g;
        }
    }
    0
}

fn main() {
    let g_a = primitive_root(P);
    let g_b = primitive_root(P);

    let alice_private = private_key();
    let bob_private = private_key();

    let alice_public = public_key(alice_private, g_a);
    let bob_public = public_key(bob_private, g_b);

    // Changing the secrets between Alice and Bob
    let alice_shared_secret = shared_secret(alice_private, bob_public);
    let bob_shared_secret = shared_secret(bob_private, alice_public);

    println!("Shared secret Alice: {}", alice_shared_secret);
    println!("Shared secret Bob: {}", bob_shared_secret);

    println!("Alice private key: {}", alice_private);
    println!("Bob private key: {}", bob_private);
    println!("Alice public key: {}", alice_public);
    println!("Bob public key: {}", bob_public);
}
