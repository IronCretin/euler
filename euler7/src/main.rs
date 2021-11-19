#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

/// nth prime number
pub fn nth_prime(n: u64) -> u64 {
    let mut primes = Vec::with_capacity(n as _);
    'outer: for i in 2.. {
        if primes.len() >= n as _ {
            break;
        }
        for p in &primes {
            if i % p == 0 {
                continue 'outer;
            }
        }
        primes.push(i);
    }
    primes.last().copied().unwrap_or(0)
}

fn main() {
    println!("{}", nth_prime(10001));
}

common::tests! {
    nth_prime;
    6 => 13;
    10001 => 104743;
}
