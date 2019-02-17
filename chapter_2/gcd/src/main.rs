use std::io::Write;     //for the writeln! macro
use std::str::FromStr;  //for the u64::from_str function

fn main() {
    let mut numbers = Vec::new();   //new mutable vector of type u64
    for arg in std::env::args().skip(1) {   //create iterator, skip name of program
        //parse argument as u64, from_str returns Result indicating success or
        //error (Ok(v) or Err(e)
        numbers.push(u64::from_str(&arg)
                     .expect("error parsing argument"));
    }

    //error out if numbers has no content
    if numbers.len() == 0 {
        //writeln! writes out error message to stderr output stream
        //unwrap checks writing error didn't fail
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0]; //initialize gcd to first number
    for m in &numbers[1..] {    //& borrows reference to numbers for looping
        d = gcd(d, *m); //* gets value referred to by m
    }

    println!("The greatest common divisor of {:?} is {}",
             numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}
