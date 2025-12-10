
#[macro_export]
macro_rules! day {
    {{$p1:ident <- $p1i:expr => $p1o:expr ; $p1ot:ty}} => {
        pub fn do_part1(input: &str) -> Result<$p1ot, ()> {
            std::panic::catch_unwind(|| {
                $p1(input)
            }).map_err(|_| ())
        }
        pub fn do_part2(input: &str) -> Result<$p1ot, ()> {
            Ok($p1o)
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            const PART1_TEST_INPUT: &str = $p1i;
            const PART1_TEST_OUTPUT: $p1ot = $p1o;

            #[test]
            fn test_part1() {
                let res = do_part1(PART1_TEST_INPUT);
                assert_eq!(res, Ok(PART1_TEST_OUTPUT));
            }
        }
    };
    {{$p1:ident <- $p1i:expr => $p1o:expr ; $p1ot:ty},
    {$p2:ident <- $p2i:expr => $p2o:expr ; $p2ot:ty}} => {
        pub fn do_part1(input: &str) -> Result<$p1ot, ()> {
            std::panic::catch_unwind(|| {
                $p1(input)
            }).map_err(|_| ())
        }
        pub fn do_part2(input: &str) -> Result<$p2ot, ()> {
            std::panic::catch_unwind(|| {
                $p2(input)
            }).map_err(|_| ())
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            const PART1_TEST_INPUT: &str = $p1i;
            const PART1_TEST_OUTPUT: $p1ot = $p1o;
            const PART2_TEST_INPUT: &str = $p2i;
            const PART2_TEST_OUTPUT: $p2ot = $p2o;

            #[test]
            fn test_part1() {
                let res = do_part1(PART1_TEST_INPUT);
                assert_eq!(res, Ok(PART1_TEST_OUTPUT));
            }
            #[test]
            fn test_part2() {
                let res = do_part2(PART2_TEST_INPUT);
                assert_eq!(res, Ok(PART2_TEST_OUTPUT));
            }
        }
    }
}