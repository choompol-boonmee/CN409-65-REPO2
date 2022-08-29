use std::io::Write;
use std::path::Path;
use std::fs::File;

fn main() {
	let a = Path::new("text.txt");
//	let mut a = File::create(a).unwrap();
//	a.write_all("ABCDE".as_bytes()).unwrap();

	let mut a = File::create(a).expect("ERROR1");
	a.write_all("ABCDE".as_bytes()).expect("ERROR2");

//	if let Ok(mut a) = a {
//	}
/*
	let mut orgf = File::create(orgp).expect("Unable to create file");
	orgf.write_all(ors.as_bytes()).expect("Unable to write data");
    println!("Hello, world!");
*/
}
