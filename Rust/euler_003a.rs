#!/usr/local/bin/rust run

/**
 The prime factors of 13195 are 5, 7, 13 and 29.

  What is the largest prime factor of the number 600851475143 ?
*/

extern mod extra;

use std::*;

fn check_prime(number: uint, known_primes: &[uint]) -> bool{  //This function borrows the pointer to the list of primes to prevent it being copied.
	for known_primes.each |&value| {
		if number % value == 0{
			return false;
		}
	}

	return true;
}

fn find_primes_to(upper_limit: uint) -> ~[uint]{
	let mut primes: ~[uint] = ~[];
	for uint::range(2, upper_limit) |value| {
		if check_prime(value, primes){
			primes.push(value);
			//println(fmt!("Prime: %u\r" ,value));
		}
	}

	return primes;
}

fn check_is_factor(potential_factor: uint, of: uint) -> bool{
	if of % potential_factor == 0{
		true
	}else{
		false
	}
}

fn main(){
	let startnum = 600851475143;

	let mut largest_result = 0;

	let primes = find_primes_to(float::sqrt(startnum as float) as uint);

	println(fmt!("Largest Prime: %u", primes[primes.len()-1]));

	for primes.each |prime| {
		if check_is_factor(*prime, startnum){
			if *prime > largest_result{
				largest_result = *prime;
			}
		}
	}

	println(fmt!("The Largest Factor is: %u", largest_result));
}