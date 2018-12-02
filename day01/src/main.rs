use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
	let filename = "./src/input.txt";

	println!("In file {}", filename);

	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();

	f.read_to_string(&mut contents)
		.expect("Something went wrong");

	// 
	let mut frequencies = HashSet::new();

	frequencies.insert(0);

	let strNums : Vec<i32> = contents.split("\n")
		.map(|s| s.parse::<i32>().unwrap())
		.collect();

	let mut finalSum : i32 = 0;
	let mut foundFrequency = false;

	while !foundFrequency {
		let some = strNums.clone();

		for num in some {
			finalSum += num;
			println!("{}", finalSum);

			if frequencies.contains(&finalSum) {
				foundFrequency = true;
				break;
			}

			frequencies.insert(finalSum);
		}
	}

	println!("--  {}", finalSum);
}
