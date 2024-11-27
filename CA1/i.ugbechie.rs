use std::io;

fn main(){
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();
	let mut input6 = String::new();
	let mut input7 = String::new();
	let mut input8 = String::new();

	println!("Enter name");
	io::stdin().read_line(&mut input1).expect("Failed to read input");

	println!("Enter date of birth");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let date of birth:u32 = input.trim().parse().expect("Failed to read input parse");

	println!("Enter email address");
	io::stdin().read_line(&mut input3).expect("Failed to read input");

	println!("Enter phone number",);
	io::stdin().read_line(&mut input4).expect("Failed to read input");
	let phone number:u64 = input.trim().parse().expect("Failed to read input parse");

	println!("Enter number of siblings");
	io::stdin().read_line(&mut input5).expect("Failed to read input");
	let number of siblings:i32 = trim().parse().expect("Failed to read input parse");

	println!("Enter number of children");
	io::stdin().read_line(&mut input6).expect("Failed to read input");
	let number of children:i32 = input.trim().parse().expect("Failed to read input parse");

	println!("Enter medical diagnosis");
	io::stdin().read_line(&mut input7).expect("Failed to read input");

	println!("Enter village of residence");
	io::stdin().read_line(&mut input8).expect("Failed to read input");

	if medical diagnosis is alzheimer && age > 50 && number of children > 4 && village of residence is akpabom;
	{
		println!("amount = 960000",);
	}
	else if medical diagnosis is arrhythmia && age = 30 && number of siblings > 4 && village of residence is ngbauji;
	{
		println!("amount = 522500",);
	}
	else if medical diagnosis is chronic kidney disease && age = 40 && number of children > 3 && number of children > 3 && village of residence is atabrikang;
	{
		println!("amount = 1275000");
	}
	else if medical diagnosis is diabetes && age > 28 && age < 45 && number of childen is from 2 to 4 && village of residence is okorobilom;
	{
		println!("amount = 720000");
	}
	else if medical diagnosis is arthritis && age > 58 && number of siblings > 5 && number of children > 5 && village of residence is emeremen;
    {
	    println!("amount = 405000");
	}
	else
	{
		println!("amount remains normal") // conditions apply to the first 100 patients that visit the clinic each day;
    }
}