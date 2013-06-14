#!/usr/local/bin/rust run

/**
 The prime factors of 13195 are 5, 7, 13 and 29.

  What is the largest prime factor of the number 600851475143 ?
*/

extern mod extra;

use std::*;

fn main(){
	let startnum = 13195;

	let mut result = startnum;
	let mut value = 1;

	loop{
		value += 1;
		if value >= result{
			break;
		}

		while result % value == 0{
			println(fmt!("%u is divisible by %u", result, value));
			result = result / value;
		}
	}

	println(fmt!("The Largest Factor is: %u", result));
}