use paste::paste;
macro_rules! solution_day {
    ($day:ident,$part1:expr,$part2:expr) => {
        paste! {
            #[test]
            fn [<solution_ $day>]() {
            assert_cmd::Command::cargo_bin(stringify!($day))
                .unwrap()
                .pipe_stdin(format!("inputs/{}/input", stringify!($day)))
                .unwrap()
                .assert()
                .success()
                .stdout(concat!($part1, "\n", $part2, "\n"));
            }
        }
    };
}

solution_day!(day1, "2031679", "19678534");
solution_day!(day2, "572", "612");
solution_day!(day3, "188116424", "104245808");
solution_day!(day4, "2507", "1969");
solution_day!(day5, "5732", "4716");
solution_day!(day6, "5329", "2162");
solution_day!(day7, "66343330034722", "637696070419031");
solution_day!(day8, "394", "1277");
