#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub degree: u8,
    pub coefficient: f64,
}

use std::error::Error;

impl Term {
    pub fn parse(term: &str) -> Result<Self, Box<dyn Error>> {
        let asterisks = ['\u{002A}', '\u{2217}', '\u{2731}', '\u{204E}'];

        // Check asterisk syntax ok before splitting
        if asterisks
            .iter()
            .any(|&c| term.starts_with(c) || term.ends_with(c))
        {
            return Err("syntax: stray '*' at start or end of term!".into());
        }

        if term.contains("**") {
            return Err("syntax: consecutive '*' found!".into());
        }

        let mut components: Vec<&str> = term
            .split(|c| asterisks.contains(&c))
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();

        // Handle free entries
        match components.len() {
            1 if components[0].contains("-X") => components.insert(0, "-1"), // '-X', coefficient = -1
            1 if components[0].contains('X') => components.insert(0, "1"),   // 'X', coefficient = 1
            1 => components.push("X^0"), // No 'X', only number, degree = 0
            2 => {}
            _ => return Err("syntax: invalid term structure!".into()),
        }

        // Guaranteed to have two components now
        let (coef_str, x_str) = (
            components[0],
            components[1].strip_prefix('-').unwrap_or(components[1]), //
        );
        let x_str = if x_str == "X" { "X^1" } else { x_str }; // Normalize "X"

        if coef_str.len() > 15 {
            return Err("number too long for this program to perserve precision".into());
        }

        let degree_str = x_str
            .strip_prefix("X^")
            .ok_or("syntax: expected prefix 'X^'")?;

        let coefficient: f64 = match coef_str.parse::<f64>() {
            Ok(val) if val.is_finite() => val,
            // Could either be format or Nan/inf
            _ => return Err(format!("invalid coefficient '{coef_str}'").into()),
        };
        let degree: u8 = match degree_str.parse::<u8>() {
            Ok(val) => val,
            _ => return Err(format!("invalid degree '{degree_str}'").into()),
        };

        Ok(Self {
            coefficient,
            degree,
        })
    }

    pub fn to_full_form(&self) -> String {
        if self.coefficient >= 0.0 {
            format!("{} * X^{}", self.coefficient, self.degree)
        } else if self.degree == 0 {
            // First to display, no space
            format!("-{} * X^{}", -self.coefficient, self.degree)
        } else {
            format!("- {} * X^{}", -self.coefficient, self.degree)
        }
    }

    pub fn to_free_form(&self) -> String {
        if self.degree == 0 {
            return format!("{}", self.coefficient);
        }

        let var = if self.degree == 1 {
            "X".to_string()
        } else {
            format!("X^{}", self.degree)
        };

        match self.coefficient {
            1.0 => var,
            -1.0 => format!("- {}", var),
            c if c < 0.0 => format!("- {} * {}", -c, var),
            c => format!("{} * {}", c, var),
        }
    }
}

#[cfg(test)]
mod term_tests {
    use super::*;

    #[test]
    fn parses_basic_term() {
        let term = Term::parse("3 * X^2").unwrap();
        assert_eq!(term.coefficient, 3.0);
        assert_eq!(term.degree, 2);
    }

    #[test]
    fn parses_implicit_coefficient() {
        let term = Term::parse("X^2").unwrap();
        assert_eq!(term.coefficient, 1.0);
        assert_eq!(term.degree, 2);
    }

    #[test]
    fn parses_number_only() {
        let term = Term::parse("42").unwrap();
        assert_eq!(term.coefficient, 42.0);
        assert_eq!(term.degree, 0);
    }

    #[test]
    fn rejects_invalid_term() {
        assert!(Term::parse("5*").is_err());
        assert!(Term::parse("*x^2").is_err());
        assert!(Term::parse("x^^2").is_err());
        assert!(Term::parse("abc").is_err());
    }

    #[test]
    fn rejects_nan_inf() {
        assert!(Term::parse("inf*X^2").is_err());
        assert!(Term::parse("NaN*X^2").is_err());
    }
    #[test]
    fn full_and_free_form_output() {
        let term = Term {
            coefficient: -1.0,
            degree: 1,
        };
        assert_eq!(term.to_full_form(), "- 1 * X^1");
        assert_eq!(term.to_free_form(), "- X");

        let term = Term {
            coefficient: 2.0,
            degree: 3,
        };
        assert_eq!(term.to_full_form(), "2 * X^3");
        assert_eq!(term.to_free_form(), "2 * X^3");
    }
}
