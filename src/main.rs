use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

macro_rules! help {
	($args:expr) => { println!("{} day part\n\tday\t01..25\n\tpart = 1 or 2", $args[0]); }
}


fn main() {
	let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
    	help!(args);
    } else {
    	match ( args[1].as_ref(), args[2].as_ref() ) {
    		("01","1") => day01::part1(),
    		("01","2") => day01::part2(),
    		("02","1") => day02::part1(),
    		("02","2") => day02::part2(),
    		("03","1") => day03::part1(args),
    		("03","2") => day03::part2(args),
    		("04","1") => day04::part1(args),
    		("04","2") => day04::part2(args),
    		("05","1") => day05::part1(args),
    		("05","2") => day05::part2(args),
    		("06","1") => day06::part1(args),
    		("06","2") => day06::part2(args),
    		("07","1") => day07::part1(args),
    		("07","2") => day07::part2(args),
    		("08","1") => day08::part1(args),
    		("08","2") => day08::part2(args),
    		("09","1") => day09::part1(args),
    		("09","2") => day09::part2(args),
    		("10","1") => day10::part1(args),
    		("10","2") => day10::part2(args),
    		("11","1") => day11::part1(args),
    		("11","2") => day11::part2(args),
    		("12","1") => day12::part1(args),
    		("12","2") => day12::part2(args),
    		("13","1") => day13::part1(args),
    		("13","2") => day13::part2(args),
    		("14","1") => day14::part1(args),
    		("14","2") => day14::part2(args),
    		("15","1") => day15::part1(args),
    		("15","2") => day15::part2(args),
    		("16","1") => day16::part1(args),
    		("16","2") => day16::part2(args),
    		("17","1") => day17::part1(args),
    		("17","2") => day17::part2(args),
    		("18","1") => day18::part1(args),
    		("18","2") => day18::part2(args),
    		("19","1") => day19::part1(args),
    		("19","2") => day19::part2(args),
    		("20","1") => day20::part1(args),
    		("20","2") => day20::part2(args),
    		("21","1") => day21::part1(args),
    		("21","2") => day21::part2(args),
    		("22","1") => day22::part1(args),
    		("22","2") => day22::part2(args),
    		("23","1") => day23::part1(args),
    		("23","2") => day23::part2(args),
    		("24","1") => day24::part1(args),
    		("24","2") => day24::part2(args),
    		("25","1") => day25::part1(args),
    		("25","2") => day25::part2(args),
    		_ => unimplemented!()
    	}
    }
}
