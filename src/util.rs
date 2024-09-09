macro_rules! rounddown {
    ($x:expr, $y:expr) => {
        ($x / $y) * $y
    };
}
pub(crate) use rounddown;

macro_rules! to_index {
    ($x:expr) => {
        usize::try_from($x & 0xff).unwrap()
    };
}
pub(crate) use to_index;

#[cfg(test)]
mod tests {
    #[test]
    fn test_rounddown() {
        assert_eq!(rounddown!(1, 1), 1);
        assert_eq!(rounddown!(2, 1), 2);
        assert_eq!(rounddown!(1023, 512), 512);
        assert_eq!(rounddown!(1024, 512), 1024);
        assert_eq!(rounddown!(1025, 512), 1024);
    }

    #[test]
    fn test_to_index() {
        assert_eq!(to_index!(0), 0);
        assert_eq!(to_index!(1), 1);

        assert_eq!(to_index!(1u8), 1);
        assert_eq!(to_index!(1u16), 1);
        assert_eq!(to_index!(1u32), 1);
        assert_eq!(to_index!(1u64), 1);
        assert_eq!(to_index!(1usize), 1);

        assert_eq!(to_index!(255u8), 255);
        assert_eq!(to_index!(255u16), 255);
        assert_eq!(to_index!(255u32), 255);
        assert_eq!(to_index!(255u64), 255);
        assert_eq!(to_index!(255usize), 255);
    }
}
