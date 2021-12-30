pub fn run() {
	let name = "Ryan";

	let mut age = 23;
	println!("Variables are immutable but we can set mut and change my age from {}", age);
	age = 22;
	println!("My name is {} and I am {} years old", name, age);

	//const type has to be defined
	const ID: i32 = 001;
	println!("ID: {}", ID);

	//assigning multiple variables at once
	let (my_name, my_age) = ("Ryan", 23);
}