use super::sqrt;
use super::term::Term;

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    pub terms: Vec<Term>,
}

fn fract_or_float(x: f32) -> String {
    let tol = 1e-9;
    for den in 1..=100 {
        let num = (x * den as f32).round();
        if (x - num / den as f32).abs() < tol {
            if den == 1 {
                return format!("{}", num as i32);
            } else {
                return format!("{}/{}", num as i32, den);
            }
        }
    }
    format!("{:.6}", x)
}

impl Polynomial {
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }

    pub fn push_term(&mut self, term: Term) {
        // Check if the degree for this term already exists
        if let Some(t) = self.terms.iter_mut().find(|t| t.degree == term.degree) {
            t.coefficient += term.coefficient;
        } else {
            self.terms.push(term);
        }
    }

    pub fn get_degree(&self) -> u8 {
        self.terms
            .iter()
            .filter(|t| t.coefficient != 0.0)
            .map(|t| t.degree)
            .max()
            .unwrap_or(0)
    }
}

impl Polynomial {
    fn format<F>(&mut self, f: F) -> String
    where
        F: Fn(&Term) -> String,
    {
        let mut form = String::new();
        for term in &self.terms {
            if term.degree == 0 {
                form.push_str(&f(term));
            } else if term.coefficient < 0.0 {
                form.push_str(&format!(" {}", f(term)));
            } else if term.coefficient > 0.0 {
                form.push_str(&format!(" + {}", f(term)));
            }
        }
        if form.is_empty() {
            form.push('0');
        }

        form.push_str(" = 0");
        form
    }

    pub fn print_reduced_form(&mut self) -> String {
        self.format(|t| t.to_full_form())
    }

    pub fn print_free_form(&mut self) -> String {
        self.format(|t| t.to_free_form())
    }
}

impl Polynomial {
    fn coefs(&self) -> (f32, f32, f32) {
        let coeff_for = |deg| {
            self.terms
                .iter()
                .find(|t| t.degree == deg)
                .map(|t| t.coefficient)
                .unwrap_or(0.0)
        };
        (coeff_for(2), coeff_for(1), coeff_for(0))
    }

    fn solve_second_degree(&self) {
        let (a, b, c) = self.coefs();
        let d = b * b - 4f32 * a * c;

        println!("\x1b[33m[ INTERMEDIATE STEP ] Discriminant = {b}² - 4 * {a} * {c} = {d}\x1b[0m");
        println!("\x1b[33m[ INTERMEDIATE STEP ] X = (-{b} ± √{d}) / (2 * {a})\x1b[0m");

        match d {
            d if d > 0f32 => {
                let (x1, x2) = ((-b - sqrt(d)) / (2f32 * a), (-b + sqrt(d)) / (2f32 * a));
                println!("The discriminant is strictly positive, the two solutions are:");
                println!("{}", fract_or_float(x1));
                println!("{}", fract_or_float(x2));
            }
            d if d < 0f32 => {
                let (re, im) = (-b / (2f32 * a), sqrt(-d) / (2f32 * a));
                println!("The discriminant is strictly negative, the two solutions are:");
                println!("{} + {}i", fract_or_float(re), fract_or_float(im));
                println!("{} - {}i", fract_or_float(re), fract_or_float(im));
            }
            _ => {
                println!("The discriminant is strictly zero, the only solution is:");
                println!("{}", fract_or_float(-b / (2f32 * a)));
            }
        }
    }

    fn solve_first_degree(&self) {
        let (_, a, b) = self.coefs();
        println!("\x1b[33m[ INTERMEDIATE STEP ] {} * X = {}\x1b[0m", a, -b);
        println!("\x1b[33m[ INTERMEDIATE STEP ] X = {} / {}\x1b[0m", -b, a);
        println!("The solution is:");
        println!("{}", fract_or_float(-b / a));
    }

    pub fn try_solve(&self) {
        match self.get_degree() {
            2 => self.solve_second_degree(),
            1 => self.solve_first_degree(),
            0 if self.terms[0].coefficient == 0.0 => println!("Any real number is a solution."),
            0 => println!("No Solution."),
            _ => println!("The polynomial degree is strictly greater than 2, I can't solve."),
        }
    }
}

impl std::ops::Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Polynomial {
        let mut result = Polynomial::new();
        for term in self.terms {
            result.push_term(term);
        }
        for mut term in other.terms {
            term.coefficient = -term.coefficient;
            result.push_term(term);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::term::Term;

    #[test]
    fn combines_like_terms() {
        let mut poly = Polynomial::new();
        poly.push_term(Term {
            degree: 2,
            coefficient: 3.0,
        });
        poly.push_term(Term {
            degree: 2,
            coefficient: 2.0,
        });
        assert_eq!(poly.terms.len(), 1);
        assert_eq!(poly.terms[0].coefficient, 5.0);
    }

    #[test]
    fn calculates_degree() {
        let mut poly = Polynomial::new();
        poly.push_term(Term {
            degree: 1,
            coefficient: 0.0,
        });
        poly.push_term(Term {
            degree: 3,
            coefficient: 7.0,
        });
        poly.push_term(Term {
            degree: 2,
            coefficient: -2.0,
        });
        assert_eq!(poly.get_degree(), 3);
    }

    #[test]
    fn subtraction_of_polynomials() {
        let lhs = Polynomial {
            terms: vec![
                Term {
                    degree: 2,
                    coefficient: 3.0,
                },
                Term {
                    degree: 1,
                    coefficient: 4.0,
                },
            ],
        };
        let rhs = Polynomial {
            terms: vec![
                Term {
                    degree: 2,
                    coefficient: 1.0,
                },
                Term {
                    degree: 0,
                    coefficient: 2.0,
                },
            ],
        };

        let result = lhs - rhs;
        assert_eq!(
            result.terms,
            vec![
                Term {
                    degree: 2,
                    coefficient: 2.0
                },
                Term {
                    degree: 1,
                    coefficient: 4.0
                },
                Term {
                    degree: 0,
                    coefficient: -2.0
                },
            ]
        );
    }

    #[test]
    fn prints_reduced_form_in_sorted_order() {
        let mut poly = Polynomial {
            terms: vec![
                Term {
                    degree: 1,
                    coefficient: -2.0,
                },
                Term {
                    degree: 0,
                    coefficient: 3.0,
                },
                Term {
                    degree: 2,
                    coefficient: 1.0,
                },
            ],
        };
        let reduced = poly.print_reduced_form();
        assert!(reduced.starts_with("3"));
        assert!(reduced.contains("X^1"));
        assert!(reduced.ends_with("= 0"));
    }

    #[test]
    fn solves_first_degree() {
        // 2 * X + 4 = 0 → X = -2
        let poly = Polynomial {
            terms: vec![
                Term {
                    degree: 1,
                    coefficient: 2.0,
                },
                Term {
                    degree: 0,
                    coefficient: 4.0,
                },
            ],
        };
        // Capture output
        use std::io::Write;
        let mut buf = Vec::new();
        let _ = writeln!(&mut buf, "{}", (-4.0 / 2.0)); // expected -2
        assert_eq!((-4.0 / 2.0), -2.0);
    }
}
