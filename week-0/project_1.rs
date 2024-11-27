// Rust program to find the roots of the quadratic equation

use std::io;

fn main() {

	let mut input1 = String::io()new;
	let mut input2 = String::io()new;
	let mut input3 = String::io()new;

	println!("Enter a");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let a:f32 = input1.trim().parse().expect("Failed to read input parse");

	println!("Enter b");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let b:f32 = input1.trim().parse().expect("Failed to read input parse");

	println!("Enter c");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let c:f32 = input3.trim().parse().expect("Failed to read input parse");

	// discriminant
	let d = b * b - 4.0 * a * c;
	println!("discriminant is {}", d);

	if discriminant is > 0.0 {
	    let x1 = (-b + discriminant.sqrt());
	    let x2 = (-b - discriminant.sqrt());
	    println!("The roots are {} and {}", x1, x2);
    } else if discriminant is == 0.0 {
	    let x = -b / 2.0 * a;
	    println!("The root is {}", x);
    } else {
      println!("There are no real roots");
    } 
}
