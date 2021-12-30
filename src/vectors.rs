pub fn run() {
	let mut numbers : Vec<i32> = vec![1, 2, 3, 4, 5];

	println!("{:?}", numbers);
	 
	numbers[1] = 0;
	//index the array
	println!("{}", numbers[1]);
	numbers.append(&mut vec![9]);
	numbers[5] = 9;
	//arrays are stack allocated 
	println!("{:?}", numbers);
	println!("This vector occupies {} bytes", std::mem::size_of_val(&numbers));
	
	//get slices of arrays
	let slice: &[i32] = &numbers[0..2];
	println!("Slice: {:?}", slice);
}