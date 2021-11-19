#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

/// finds the largest palindrome product of numbers of n digits
pub fn largest_pal_prod(n: u32) -> u64 {
    let mut buf = Vec::with_capacity(10_usize.pow(n * 2));
    let mut largest = 0;
    for i in (1..10_u64.pow(n)).rev() {
        for j in (1..=i).rev() {
            let p = i * j;
            if p > largest {
                buf.clear();
                let mut rem = p;
                while rem != 0 {
                    buf.push(rem % 10);
                    rem /= 10;
                }
                if buf.iter().zip(buf.iter().rev()).all(|(c, d)| c == d) {
                    largest = p;
                }
            } else {
                break;
            }
        }
    }
    largest
}

fn main() {
    println!("{}", largest_pal_prod(3));
}

common::tests! {
    largest_pal_prod;
    2 => 9009;
    3 => 906609;
}
