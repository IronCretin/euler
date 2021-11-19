#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// smallest number divisible by each number up to n without remainder
pub fn smallest_multiple(n: u64) -> u64 {
    let mut num = 1;
    for i in 2..n {
        num *= i / gcd(num, i);
    }
    num
}

fn main() {
    println!("{}", smallest_multiple(20));
}

common::tests! {
    smallest_multiple;
    10 => 2520;
    20 => 232792560;
}
