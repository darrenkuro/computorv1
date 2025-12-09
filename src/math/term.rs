use std::fmt;

#[derive(Debug)]
pub struct Term {
    pub degree: u8,
    pub coefficient: f32,
}

impl Term {
    pub fn new(term: &str) -> Result<Self, String> {
        let asterisks = ['\u{002A}', '\u{2217}', '\u{2731}', '\u{204E}'];
        let mut components: Vec<&str> = term
            .split(|c| asterisks.contains(&c))
            .filter(|s| !s.trim().is_empty())
            .collect();

        match components.len() {
            1 if components[0].contains('X') => components.insert(0, "1"), // 'X', coefficient = 1
            1 => components.push("X^0"), // No 'X', only number, degree = 0
            2 => {}
            _ => return Err("Syntax Error: invalid term structure!".to_string()),
        }

        // Guaranteed to have two components now
        let (coef_str, var_str) = (components[0].trim(), components[1].trim());
        let var_str = if var_str == "X" { "X^1" } else { var_str }; // Normalize "X"
        let degree_str = var_str
            .strip_prefix("X^")
            .ok_or("Syntax Error: expected prefix 'X^'")?;

        let coefficient: f32 = coef_str
            .parse()
            .map_err(|_| format!("Syntax Error: invalid coefficient '{coef_str}'"))?;
        let degree: u8 = degree_str
            .parse()
            .map_err(|_| format!("Syntax Error: invalid degree '{degree_str}'"))?;

        Ok(Self {
            coefficient,
            degree,
        })
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coefficient >= 0.0 {
            write!(f, "{} * X^{}", self.coefficient, self.degree)
        } else {
            write!(f, "- {} * X^{}", -self.coefficient, self.degree)
        }
    }
}

#[cfg(test)]
mod term_tests {
    #[test]
    fn print() {
        // let mut term = Term::new(10, 1.1);
        // let mut buf = Vec::new();
        // write!(buf, "{term}").unwrap();
        // assert_eq!(String::from_utf8(buf).unwrap(), "1.1 * X^10");

        // buf = Vec::new();
        // term = Term::new(10, 1.0);
        // write!(buf, "{term}").unwrap();
        // assert_eq!(String::from_utf8(buf).unwrap(), "1 * X^10");

        // buf = Vec::new();
        // term = Term::new(10, -1.0);
        // write!(buf, "{term}").unwrap();
        // assert_eq!(String::from_utf8(buf).unwrap(), "1 * X^10");
    }
}
