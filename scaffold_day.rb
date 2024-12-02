# frozen_string_literal: true

require 'fileutils'

if ARGV.length != 1
  puts 'Usage: ruby scaffold_day.rb <DAY_NUMBER>'
  exit 1
end

day_nr = ARGV[0]

puts 'Creating input directory...'
FileUtils.mkdir_p("inputs/day#{day_nr}")

puts 'Creating scaffold Rust file...'
File.open("src/bin/day#{day_nr}.rs", 'w') do |f|
  f << <<~HEREDOC
    fn main() {
        println!("{}", part1());
        println!("{}", part2());
    }

    fn part1() -> i32 {
      unimplemented!();
    }
    fn part2() -> i32 {
      unimplemented!();
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_examples() {
        }
    }
  HEREDOC
end

puts 'Done! Happy Advent!'
