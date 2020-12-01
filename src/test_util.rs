pub fn load_input_for_day(day: u8) -> String {
    std::fs::read_to_string(format!("input/2020/day{}.txt", day))
        .unwrap()
        .trim()
        .to_string()
}

#[macro_export]
macro_rules! tests {
    ($day:expr, $part1:expr, $part2:expr) => {
        #[test]
        fn test_part1() {
            let input = crate::test_util::load_input_for_day($day);

            let value = super::gather_input(&input);

            let actual = super::part1(&value);

            assert_eq!(actual, $part1);
        }

        #[test]
        fn test_part2() {
            let input = crate::test_util::load_input_for_day($day);

            let value = super::gather_input(&input);

            let actual = super::part2(&value);

            assert_eq!(actual, $part2);
        }
    };
}

#[macro_export]
macro_rules! test_examples {
    (($($input_part1:expr => $output_part1:expr),* $(,)?), ($($input_part2:expr => $output_part2:expr),* $(,)?)) => {
        #[test]
        fn test_part1_examples() {
            $(
                let value = super::gather_input($input_part1);

                let actual = super::part1(&value);

                assert_eq!(actual, $output_part1);
            )*
        }

        #[test]
        fn test_part2_examples() {
            $(
                let value = super::gather_input($input_part2);

                let actual = super::part2(&value);

                assert_eq!(actual, $output_part2);
            )*
        }
    }
}
