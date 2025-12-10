use crate::math::polynomial::Polynomial;
use crate::math::term::Term;
use std::error::Error;

pub fn parse(args: &str) -> Result<Polynomial, Box<dyn Error>> {
    let args = args.trim().replace("x", "X"); // Accept both x and X

    if args.is_empty() {
        return Err("empty input!".into());
    }

    if !args.contains("X") {
        return Err("no unknown variable X (or x) to solve!".into());
    }

    // Divide into left and right sides
    let sides: Vec<&str> = args.split('=').collect();
    match sides.len() {
        2 => {} // Continue
        1 => return Err("syntax: missing '=' symbol!".into()),
        _ => return Err("syntax: too many '=' symbols!".into()),
    }

    // Reformat - sign for easy processing
    let lhs = sides[0].replace("-", "+ -").replace("- ", "-");

    let terms: Vec<&str> = lhs.split('+').collect();
    if terms.is_empty() {
        return Err("syntax: left side is empty!".into());
    }

    let mut lhs = Polynomial::new();
    for term in terms {
        match Term::parse(term) {
            Ok(term) => lhs.push_term(term),
            Err(e) => return Err(e.into()),
        }
    }

    let rhs = sides[1].replace("-", "+ -").replace("- ", "-");
    let terms: Vec<&str> = rhs.split('+').collect();
    if terms.is_empty() {
        return Err("syntax: right side is empty!".into());
    }
    let mut rhs = Polynomial::new();
    for term in terms {
        match Term::parse(term) {
            Ok(term) => rhs.push_term(term),
            Err(e) => return Err(e.into()),
        }
    }
    let mut res = lhs - rhs;
    res.terms.sort_by_key(|t| t.degree);

    Ok(res)
}
