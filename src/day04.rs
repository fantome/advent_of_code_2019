
macro_rules! println_day {
	($part:expr) => {
		println!("--- Day 4: Secure Container ---");
		println!("--- Part {}", $part);
	};
}
macro_rules! help {
	($args:expr) => {{
		println!("{} day part inputs\n\tday\t01..25\n\tpart = 1 or 2", $args[0]);
		println!("\tinputs: ");
		return
	}}
}

fn is_possible_password_v1(password: &str) -> bool {
	let mut has_double_digit = false;
	let pw_vec:Vec<char> = password.chars().collect();
	for i in 0..password.len()-1 {
		let a = pw_vec[i];
		let b = pw_vec[i+1];
		if a == b {
			has_double_digit = true
		} else if a > b {
			return false
		}
	}
	has_double_digit
}

pub fn part1(args: Vec<String>) {
    println_day!("1");
    if args.len() != 4 { help!(args); }

    let inputs: (usize, usize);
    match args[3].as_ref() {
    	"test1" => inputs = (10, 30),
    	"input" => inputs = (272091, 815432),
    	_ => help!(args)
    }

    let mut cnt_pw = 0;
    for i in inputs.0 .. inputs.1+1 {
    	let s = format!("{:06}", i);
    	if is_possible_password_v1(&s) {
    		// println!("{}", s);
    		cnt_pw += 1;
    	}
    }

    println!("\n==> Count valid password in range [{},{}] = {}", inputs.0, inputs.1, cnt_pw);
}


fn is_possible_password_v2(password: &str) -> bool {
	let mut has_double_digit = false;
	let pw_vec:Vec<char> = password.chars().collect();
	let mut multiple_digit_val = pw_vec[0];
	let mut multiple_digit_cnt = 1;

	for i in 0..password.len()-1 {
		let a = pw_vec[i];
		let b = pw_vec[i+1];
		if a == b {
			if a == multiple_digit_val {
				multiple_digit_cnt += 1;
			} else {
				if multiple_digit_cnt == 2 {
					has_double_digit = true
				}
				multiple_digit_val = a;
				multiple_digit_cnt = 2;
			}
		} else if a > b {
			return false
		}
	}
	has_double_digit || multiple_digit_cnt == 2
}

pub fn part2(args: Vec<String>) {
    println_day!("2");
    if args.len() != 4 { help!(args); }

    let inputs: (usize, usize);
    match args[3].as_ref() {
    	"test1" => inputs = (10, 30),
    	"input" => inputs = (272091, 815432),
    	_ => help!(args)
    }

    let mut cnt_pw = 0;
    for i in inputs.0 .. inputs.1+1 {
    	let s = format!("{:06}", i);
    	if is_possible_password_v2(&s) {
    		// println!("{}", s);
    		cnt_pw += 1;
    	}
    }

    println!("\n==> Count valid password in range [{},{}] = {}", inputs.0, inputs.1, cnt_pw);
}
