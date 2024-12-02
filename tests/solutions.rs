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

// solution_day!(day1, "298", "158");
