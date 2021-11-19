#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

fn factorize(n: u64, cb: &mut impl FnMut(u64)) {
    for i in (2..n).take_while(|i| i * i <= n) {
        if n % i == 0 {
            factorize(i, cb);
            factorize(n / i, cb);
            return;
        }
    }
    // n is prime
    cb(n);
}
/// finds the largest prime factor of a number
pub fn largest_factor(n: u64) -> u64 {
    let mut largest = 0;
    factorize(n, &mut |f| {
        if f > largest {
            largest = f;
        }
    });
    largest
}

fn main() {
    println!("{}", largest_factor(600851475143));
}

common::tests! {
    largest_factor;
    13195 => 29;
    600851475143 => 6857;
}
