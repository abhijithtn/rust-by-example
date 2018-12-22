fn main() {
	println!("{} days", 31);

	println!("{0}, this is {1}, this is {0}", "Foo", "Bar");

	println!("{subject} {verb} {object}", 
			object="the lazy dag", 
			subject="the quick brown fox", 
			verb="jumps over");

	println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

	println!("{number:>width$}", number=1, width=6 );

	println!("{number:>0width$}", number=1, width=6);

	let pi = 3.141592;
	println!("Pi is roughly {pi:.precision$}", pi=pi, precision=3);
}