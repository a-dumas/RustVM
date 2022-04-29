pub mod bitpack;
pub use crate::bitpack::fitss;
pub use crate::bitpack::fitsu;
pub use crate::bitpack::gets;
pub use crate::bitpack::getu;
pub use crate::bitpack::news;
pub use crate::bitpack::newu;

#[cfg(test)]
mod tests {
    use crate::bitpack;
    use rand::{self, Rng};
    #[test]
    fn newu_simple() {
        let word = bitpack::newu(0_u64, 4, 0, 1);
        assert_eq!(word, Some(1_u64));
    }

    #[test]
    fn news_simple() {
        let word = bitpack::news(0_u64, 4, 0, -1);
        assert_eq!(word, Some(15_u64));
    }

    #[test]
    fn pack_and_unpack_unsigned() {
        let word = bitpack::newu(0_u64, 4, 3, 15).unwrap();
        let val = bitpack::getu(word, 4, 4);
        assert_eq!(val, 7);
    }

    #[test]
    fn pack_and_unpack_signed() {
        let word = bitpack::news(0_u64, 5, 3, -15).unwrap();
        let val = bitpack::gets(word, 5, 3);
        assert_eq!(val, -15);
    }

    #[test]
    fn pack_too_big_unsigned() {
        let word = bitpack::newu(0_u64, 3, 3, 15);
        assert_eq!(word, None);
    }

    #[test]
    fn pack_too_big_signed() {
        let word = bitpack::news(0_u64, 3, 3, -15);
        assert_eq!(word, None);
    }

    #[test]
    fn test_exhaustive_unsigned() {
        let mut rng = rand::thread_rng();
        for width in 0..64 {
            for lsb in 0..(64 - width) {
                // test 10 random values
                for _ in 0..10 {
                    let limit = (1_u128 << (width)) as u64;
                    let val = rng.gen_range(0..limit);
                    let word = bitpack::newu(0_u64, width, lsb, val).unwrap();
                    let result = bitpack::getu(word, width, lsb);
                    assert_eq!(val, result);
                }
            }
        }
    }
    #[test]
    fn test_exhaustive_signed() {
        let mut rng = rand::thread_rng();
        for width in 0..64 {
            for lsb in 0..(64 - width) {
                // test 10 random values
                for _ in 0..10 {
                    let ulimit = ((1_i128 << width) / 2 - 1) as i64;
                    let llimit = (-ulimit - 1) as i64;
                    let val = if width == 0 {
                        0
                    } else {
                        rng.gen_range(llimit..ulimit)
                    };
                    let word = bitpack::news(0_u64, width, lsb, val).unwrap();
                    let result = bitpack::gets(word, width, lsb);
                    assert_eq!(val, result);
                }
            }
        }
    }
}
