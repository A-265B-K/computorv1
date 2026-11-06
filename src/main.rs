use std::env;

enum Polynomial {
    FirstDegreePolynomial(FirstDegreePolynomial),
    SecondDegreePolynomial(SecondDegreePolynomial),
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

fn add_expression(
    expression: &str,
    side_sign: f64,
    coefficients: &mut [f64; 3],
) -> Option<()> {
    let mut term = String::new();
    let mut term_sign = 1.0;

    for character in expression.chars().chain(std::iter::once('+')) {
        if character == '+' || character == '-' {
            if term.trim().is_empty() {
                if character == '-' && term.is_empty() {
                    term_sign = -term_sign;
                    continue;
                }
                return None;
            }

            let compact_term: String =
                term.chars().filter(|ch| !ch.is_whitespace()).collect();
            let (coefficient, degree) =
                if let Some((value, variable)) = compact_term.split_once('*') {
                    if variable.contains('*') {
                        return None;
                    }
                    let coefficient = value.parse::<f64>().ok()?;
                    let (_, exponent) = variable.rsplit_once('^')?;
                    let degree = exponent.parse::<usize>().ok()?;
                    if degree > 2 || !coefficient.is_finite() {
                        return None;
                    }
                    (coefficient, degree)
                } else if let Ok(value) = compact_term.parse::<f64>() {
                    if !value.is_finite() {
                        return None;
                    }
                    (value, 0)
                } else {
                    let (_, exponent) = compact_term.rsplit_once('^')?;
                    let degree = exponent.parse::<usize>().ok()?;
                    if degree > 2 {
                        return None;
                    }
                    (1.0, degree)
                };

            coefficients[degree] += side_sign * term_sign * coefficient;
            term.clear();
            term_sign = if character == '-' { -1.0 } else { 1.0 };
        } else {
            term.push(character);
        }
    }

    Some(())
}

fn extract_simplified_form(args: &[String]) -> Option<Polynomial> {
    if args.len() != 2 {
        return None;
    }

    let mut equation_sides = args[1].split('=');
    let left_side = equation_sides.next()?;
    let right_side = equation_sides.next()?;
    if equation_sides.next().is_some() {
        return None;
    }

    let mut coefficients = [0.0; 3];
    add_expression(left_side, 1.0, &mut coefficients)?;
    add_expression(right_side, -1.0, &mut coefficients)?;

    if coefficients[2] == 0.0 {
        Some(Polynomial::FirstDegreePolynomial(FirstDegreePolynomial {
            a: coefficients[1],
            b: coefficients[0],
        }))
    } else {
        Some(Polynomial::SecondDegreePolynomial(SecondDegreePolynomial {
            a: coefficients[2],
            b: coefficients[1],
            c: coefficients[0],
        }))
    }
}

fn calculate_first_degree_polynomial(a: f64, b: f64) -> f64 {
    assert!(a != 0.0, "No solution possible");
    -b / a
}

fn root(under_sq_root: f64) -> f64 {
    assert!(under_sq_root >= 0.0, "No real solution is possible");
    if under_sq_root == 0.0 {
        return 0.0;
    }

    let mut x: f64 = under_sq_root;
    let mut y: f64 = 1.0;
    let e: f64 = 0.000_000_1;
    while (x - y) > e {
        x = (x + y) / 2.0;
        y = under_sq_root / x;
    }
    x
}

fn calculate_second_degree_polynomial(a: f64, b: f64, c: f64) -> (f64, f64) {
    let root = root(b * b - 4.0 * a * c);

    let x0 = (-b - root) / (2.0 * a);
    let x1 = (-b + root) / (2.0 * a);
    (x0, x1)
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some(equation) = extract_simplified_form(&args) else {
        eprintln!("Neither a first nor second degree polynomial");
        return;
    };
    match equation {
        Polynomial::FirstDegreePolynomial(polynomial) => {
            println!(
                "Reduced form: X^1 * {} + X^0 * {} = 0",
                polynomial.a, polynomial.b
            );
            println!("Polynomial degree: 1");

            let solution = calculate_first_degree_polynomial(polynomial.a, polynomial.b);
            println!("The solution is: {solution}");
        }
        Polynomial::SecondDegreePolynomial(polynomial) => {
            println!(
                "Reduced form: X^2 * {} + X^1 * {} + X^0 * {} = 0",
                polynomial.a, polynomial.b, polynomial.c
            );
            println!("Polynomial degree: 2");

            let solution = calculate_second_degree_polynomial(polynomial.a, polynomial.b, polynomial.c);
            if solution.0 == solution.1 {
                println!("The solution is: {0}", solution.0);
            } else {
                println!("The solution is: {0}, {1}", solution.0, solution.1);
            }
        }
    }
}
