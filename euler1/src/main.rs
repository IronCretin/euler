#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

fn sum_35(n: u32) -> u32 {
    // we want all numbers *below*
    let n = n - 1;
    let n3 = n/3;
    let n5 = n/5;
    let n15 = n/15;
    3 * n3 * (n3 + 1) / 2 + 5 * n5 * (n5 + 1) / 2 - 15 * n15 * (n15 + 1) / 2
}

fn main() {
    println!("{}", sum_35(1000));
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;

    #[test]
    fn sum10() {
        assert_eq!(sum_35(10), 23);
    }

    #[test]
    fn sum1000() {
        assert_eq!(sum_35(1000), 233168);
    }

    #[bench]
    fn bench1000(b: &mut test::Bencher) {
        b.iter(|| sum_35(black_box(1000)))
    }
}
