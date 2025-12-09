use crate::math::polynomial::Polynomial;
use crate::math::term::Term;

pub fn parse(args: &str) -> Result<Polynomial, String> {
    // Divide into left and right sides
    let sides: Vec<&str> = args.split('=').collect();
    match sides.len() {
        2 => {} // Continue
        1 => return Err("Syntax Error: missing '=' symbol!".to_string()),
        0 => return Err("Syntax Error: empty input!".to_string()),
        _ => return Err("Syntax Error: too many '=' symbols!".to_string()),
    }

    // Reformat - sign for easy processing
    let lhs = sides[0].replace("- ", "+ -");
    let terms: Vec<&str> = lhs.split('+').collect();
    if terms.is_empty() {
        return Err("Syntax Error: left side is empty!".to_string());
    }

    let mut lhs = Polynomial::new();
    for term in terms {
        match Term::new(term) {
            Ok(term) => lhs.push_term(term),
            Err(e) => return Err(e),
        }
    }

    let rhs = sides[1].replace("- ", "+ -");
    let terms: Vec<&str> = rhs.split('+').collect();
    if terms.is_empty() {
        return Err("Syntax Error: right side is empty!".to_string());
    }
    let mut rhs = Polynomial::new();
    for term in terms {
        match Term::new(term) {
            Ok(term) => rhs.push_term(term),
            Err(e) => return Err(e),
        }
    }
    Ok(lhs - rhs)
}
