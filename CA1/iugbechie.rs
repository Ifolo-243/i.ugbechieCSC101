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

	println!("Enter patient'name");
	io::stdin().read_line(&mut input1).expect("Failed to read input");

	println!("Enter patient's date of birth (dd-mm-yyyy)");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	
	println!("Enter patient's email address");
	io::stdin().read_line(&mut input3).expect("Failed to read input");

	println!("Enter patient's phone number",);
	io::stdin().read_line(&mut input4).expect("Failed to read input");

	println!("Enter patient's number of siblings");
	io::stdin().read_line(&mut input5).expect("Failed to read input");
	let patient siblings::u32 = trim().parse().expect("Failed to parse input");

	println!("Enter ppatient's number of children");
	io::stdin().read_line(&mut input6).expect("Failed to read input");
	let patient children::u32 = input.trim().parse().expect("Failed to parse input");

	println!("Enter patient's medical diagnosis");
	io::stdin().read_line(&mut input7).expect("Failed to read input");

	println!("Enter patient's village of residence");
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
		println!("normal charges apply") // conditions apply to the first 100 patients that visit the clinic each day;
    }
}