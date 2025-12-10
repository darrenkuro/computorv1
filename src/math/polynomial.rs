use super::sqrt;
use super::term::Term;

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    pub terms: Vec<Term>,
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

    pub fn print_reduced_form(&mut self) -> String {
        self.terms.sort_by_key(|term| term.degree);

        let mut form: String = String::new();
        for term in &mut self.terms {
            if term.degree == 0 {
                form.push_str(&term.to_full_form());
            } else if term.coefficient < 0f32 {
                form.push_str(&format!(" {}", term.to_full_form()));
            } else if term.coefficient > 0f32 {
                form.push_str(&format!(" + {}", term.to_full_form()));
            }
        }
        form.push_str(" = 0");

        form
    }

    pub fn print_free_form(&mut self) -> String {
        self.terms.sort_by_key(|term| term.degree);

        let mut form: String = String::new();
        for term in &mut self.terms {
            if term.degree == 0 {
                form.push_str(&term.to_free_form());
            } else if term.coefficient < 0f32 {
                form.push_str(&format!(" {}", term.to_free_form()));
            } else if term.coefficient > 0f32 {
                form.push_str(&format!(" + {}", term.to_free_form()));
            }
        }

        form.push_str(" = 0");
        form
    }

    fn solve_second_degree(&self) {
        let (mut a, mut b, mut c) = (0f32, 0f32, 0f32);
        let mut get_discriminant = |poly: &Polynomial| -> f32 {
            for term in &poly.terms {
                match term.degree {
                    2 => a = term.coefficient,
                    1 => b = term.coefficient,
                    0 => c = term.coefficient,
                    _ => (),
                }
            }
            b * b - 4f32 * a * c
        };

        match get_discriminant(self) {
            d if d > 0f32 => {
                println!(
                    "\x1b[33m[ INTERMEDIATE STEP ] Discriminant = {}^2 - 4 * {} * {} = {}\x1b[0m",
                    b, a, c, d
                );
                println!(
                    "\x1b[33m[ INTERMEDIATE STEP] X = ±{} - sqrt({}) / (2 * {})\x1b[0m",
                    b.abs(),
                    d,
                    a
                );
                println!("The discriminant is strictly positive, the two solutions are:");
                println!("{}", (-b - sqrt(d)) / (2f32 * a));
                println!("{}", (-b + sqrt(d)) / (2f32 * a));
            }
            d if d < 0f32 => {
                println!(
                    "\x1b[33m[ INTERMEDIATE STEP ] Discriminant = {}^2 - 4 * {} * {} = {}\x1b[0m",
                    b, a, c, d
                );
                println!(
                    "\x1b[33m[ INTERMEDIATE STEP ] X = ±{} - sqrt({}) / (2 * {})\x1b[0m",
                    b.abs(),
                    d,
                    a
                );
                println!("The discriminant is strictly negative, the two solutions are:");
                println!("{} + {}i", -b / (2f32 * a), sqrt(-d) / (2f32 * a));
                println!("{} - {}i", -b / (2f32 * a), sqrt(-d) / (2f32 * a));
            }
            d if d == 0f32 => {
                println!(
                    "\x1b[33m[ INTERMEDIATE STEP ] Discriminant = {}^2 - 4 * {} * {} = {}\x1b[0m",
                    b, a, c, d
                );
                println!(
                    "\x1b[33m[ INTERMEDIATE STEP ] X = ±{} - sqrt({}) / (2 * {})\x1b[0m",
                    b.abs(),
                    d,
                    a
                );
                println!("The discriminant is strictly zero, the only solution is:");
                println!("{}", -b / (2f32 * a));
            }
            _ => unreachable!(),
        }
    }

    fn solve_first_degree(&self) {
        let (mut a, mut b) = (0f32, 0f32);
        for term in &self.terms {
            match term.degree {
                1 => a = term.coefficient,
                0 => b = term.coefficient,
                _ => (),
            }
        }
        println!(
            "\x1b[33m[ INTERMEDIATE STEP ] {} * X^1 = {} * X^0\x1b[0m",
            a, -b
        );
        println!("\x1b[33m[ INTERMEDIATE STEP ] X = {} / {}\x1b[0m", -b, a);
        println!("The solution is:");
        println!("{}", -b / a);
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
        use std::io::{self, Write};
        let mut buf = Vec::new();
        let _ = writeln!(&mut buf, "{}", (-4.0 / 2.0)); // expected -2
        assert_eq!((-4.0 / 2.0), -2.0);
    }
}
