macro_rules! help {
	($args:expr) => {{
		println!("{} day part inputs\n\tday\t01..25\n\tpart = 1 or 2", $args[0]);
		println!("\tinputs: ");
		return
	}}
}
macro_rules! header {
    ($part:expr, $args:expr) => {
        println!("--- Day 5: Sunny with a Chance of Asteroids ---");
        println!("--- Part {}", $part);
        if $args.len() != 4 { help!($args); }
    };
}

pub fn part1(args: Vec<String>) {
    header!("1", args);

}

pub fn part2(args: Vec<String>) {
    header!("2", args);

}
