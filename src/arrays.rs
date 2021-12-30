pub fn run() {
	let numbers : [i32; 5] = [1, 2, 3, 4, 5];

	println!("{:?}", numbers);

	//index the array
	println!("{}", numbers[1]);

	//arrays are stack allocated 
	println!("This array occupies {} bytes", std::mem::size_of_val(&numbers));
	
	//get slices of arrays
	let slice: &[i32] = &numbers[0..2];
	println!("Slice: {:?}", slice);
}