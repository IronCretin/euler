#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

/// Finds the sum of even fibonacci numbers less than `max`
pub fn even_fibs(max: u64) -> u64 {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 2;
    while a < max {
        if a % 2 == 0 {
            sum += a;
        }
        let temp = a;
        a = b;
        b = temp + b;
    }
    sum
}

fn main() {
    println!("{}", even_fibs(4_000_000));
}

common::tests! {
    even_fibs;
    100 => 2 + 8 + 34;
    4_000_000 => 4613732;
}
