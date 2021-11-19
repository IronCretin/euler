#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

/// Finds the sum of all multiples of 3 and 5 less than n
fn sum_35(n: u64) -> u64 {
    // we want all numbers *below*
    let n = n - 1;
    let n3 = n / 3;
    let n5 = n / 5;
    let n15 = n / 15;
    3 * n3 * (n3 + 1) / 2 + 5 * n5 * (n5 + 1) / 2 - 15 * n15 * (n15 + 1) / 2
}

fn main() {
    println!("{}", sum_35(1000));
}

common::tests! {
    sum_35;
    10 => 23;
    1000 => 233168;
}
