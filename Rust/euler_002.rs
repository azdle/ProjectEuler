#!/usr/local/bin/rust run

fn main(){
	let mut sum = 0;
	let mut term0;
	let mut term1 = 0;
	let mut term2 = 1;

	loop{
		term0 = term1 + term2;
		if term0 >= 4000000{
			break;
		}else if term0 % 2 == 0{
			sum += term0
		}
		term2 = term1;
		term1 = term0;
	}

	println(fmt!("The total sum is %u", sum))
}