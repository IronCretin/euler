#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

/// smallest number divisible by each number up to n without remainder
pub fn sum_sq_difference(n: u64) -> u64 {
    n * n * (n + 1) * (n + 1) / 4 - (2 * n + 1) * n * (n + 1) / 6
}

fn main() {
    println!("{}", sum_sq_difference(100));
}

common::tests! {
    sum_sq_difference;
    10 => 2640;
    100 => 25164150;
}
