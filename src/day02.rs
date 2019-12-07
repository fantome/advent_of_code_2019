
fn run_program(mut inputs: Vec<usize>, verbose: bool) -> usize {
	let mut addr = 0;
	while inputs[addr] != 99 {
		let opcode = inputs[addr];
		let addr_a = inputs[addr+1].clone();
		let addr_b = inputs[addr+2].clone();
		let addr_res = inputs[addr+3].clone();
		let a = inputs[ addr_a ];
		let b = inputs[ addr_b ];
		let result: usize;
		if verbose { print!("[{},{},{},{}]\t", inputs[addr], addr_a, addr_b, addr_res); }
		if  opcode == 1  {
			if verbose { print!("adds"); }
			result = a + b;
			if verbose { println!("\t{} + {} = {}", a, b, result); }
		}
		else if  opcode == 2  {
			if verbose { print!("multi"); }
			result = a * b;
			if verbose { println!("\t{} * {} = {}", a, b, result); }
		}
		else {
			unimplemented!();
		}
		inputs[ addr_res ] = result;
		addr += 4;
	}
	if verbose { println!("[99]\tend of program\t{}", inputs[0]); }
	//println!("{:?}", inputs);
	inputs[0]
}

pub fn part1() {
    println!("--- Day 2: 1202 Program Alarm ---");
    println!("--- Part 1");

	// inputs test
	//let mut inputs = vec![1,9,10,3,2,3,11,0,99,30,40,50];
	// inputs P1
	let mut inputs = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,2,9,19,23,2,23,10,27,1,6,27,31,1,31,6,35,2,35,10,39,1,39,5,43,2,6,43,47,2,47,10,51,1,51,6,55,1,55,6,59,1,9,59,63,1,63,9,67,1,67,6,71,2,71,13,75,1,75,5,79,1,79,9,83,2,6,83,87,1,87,5,91,2,6,91,95,1,95,9,99,2,6,99,103,1,5,103,107,1,6,107,111,1,111,10,115,2,115,13,119,1,119,6,123,1,123,2,127,1,127,5,0,99,2,14,0,0];

	inputs[1] = 12;
	inputs[2] = 2;
	run_program(inputs, true);
}

pub fn part2() {
    println!("--- Day 2: 1202 Program Alarm ---");
    println!("--- Part 2");

	let inputs = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,2,9,19,23,2,23,10,27,1,6,27,31,1,31,6,35,2,35,10,39,1,39,5,43,2,6,43,47,2,47,10,51,1,51,6,55,1,55,6,59,1,9,59,63,1,63,9,67,1,67,6,71,2,71,13,75,1,75,5,79,1,79,9,83,2,6,83,87,1,87,5,91,2,6,91,95,1,95,9,99,2,6,99,103,1,5,103,107,1,6,107,111,1,111,10,115,2,115,13,119,1,119,6,123,1,123,2,127,1,127,5,0,99,2,14,0,0];

	for noun in 0..99 {
		for verb in 0..99 {
			let mut instruction = inputs.clone();
			instruction[1] = noun;
			instruction[2] = verb;
			let res = run_program(instruction, false);
			if res == 19690720 {
				println!("\nnoun = {} verb = {} answer = {}", noun, verb, (100*noun)+verb);
				return;
			}
		}
	}
}