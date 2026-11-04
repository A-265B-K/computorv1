use std::env::{self, Args};

enum Polynomial {
    FirstDegreePolynomial,
    SecondDegreePolynomial,
}

struct FirstDegreePolynomial {
    a: f64,
    b: f64,
}

struct SecondDegreePolynomial {
    a: f64,
    b: f64,
    c: f64,
}

fn args_are_valid(args: Vec<String>) -> bool {
    if args.len() != 2 {
        return false;
    }
    // add better validation here later


    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args_are_valid(args.clone()) {
        eprintln!("Neither a first nor second degree polynomial");
        return 1;
    }

    let equation = extract_simplified_form(args);

    dbg!(args);
}
