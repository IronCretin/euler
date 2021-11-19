#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

fn even_fibs(max: u32) -> u32 {
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;

    #[test]
    fn test100() {
        assert_eq!(even_fibs(100), 2+8+34);
    }

    #[test]
    fn test4M() {
        assert_eq!(even_fibs(4_000_000), 4613732);
    }

    #[bench]
    fn bench4M(b: &mut test::Bencher) {
        b.iter(|| black_box(even_fibs(black_box(4_000_000))))
    }
}
