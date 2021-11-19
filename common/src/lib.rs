#[macro_export]
macro_rules! tests {
    (
        $func:ident;
        $small:expr => $small_exp:expr;
        $( $big:expr => $big_exp:expr; )?
    ) => {
        #[cfg(test)]
        mod $func {
            use super::*;
            use std::hint::black_box;

            #[test]
            fn test_small() {
                assert_eq!($func($small), $small_exp);
            }
            #[bench]
            fn bench_small(b: &mut test::Bencher) {
                b.iter(|| black_box($func(black_box($small))))
            }

            $(
                #[test]
                fn test_big() {
                    assert_eq!($func($big), $big_exp);
                }

                #[bench]
                fn bench_big(b: &mut test::Bencher) {
                    b.iter(|| black_box($func(black_box($big))))
                }
            )?
        }
    };
}
