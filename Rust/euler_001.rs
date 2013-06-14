#!/usr/local/bin/rust run

use std::uint;

fn main(){
	let mut sum = 0;

	for uint::range(0, 1000) |i| {
		if i%3 == 0 || i%5 == 0 {
			sum += i;
		}
	}

	println(fmt!("The total sum is %u", sum))
}