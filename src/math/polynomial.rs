use super::sqrt;
use super::term::Term;
#[derive(Debug)]
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
        let mut degree = 0;
        for term in &self.terms {
            if term.coefficient != 0.0 && term.degree > degree {
                degree = term.degree;
            };
        }
        degree
    }

    pub fn print_form(&mut self) -> String {
        self.terms.sort_by_key(|term| term.degree);

        let mut form: String = String::new();
        for term in &mut self.terms {
            if term.degree == 0 {
                form.push_str(&format!("{term}"));
            } else if term.coefficient < 0f32 {
                form.push_str(&format!(" {term}"));
            } else if term.coefficient > 0f32 {
                form.push_str(&format!(" + {term}"));
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
                println!("The discriminant is strictly positive, the two solutions are:");
                println!("^ Discriminant = {}^2 - 4 * {} * {} = {}", b, a, c, d);
                println!("^ X = ±{} - sqrt({}) / (2 * {})", b.abs(), d, a);
                println!("{}", (-b - sqrt(d)) / (2f32 * a));
                println!("{}", (-b + sqrt(d)) / (2f32 * a));
            }
            d if d < 0f32 => {
                println!("The discriminant is strictly negative, the two solutions are:");
                println!("^ Discriminant = {}^2 - 4 * {} * {} = {}", b, a, c, d);
                println!("^ X = ±{} - sqrt({}) / (2 * {})", b.abs(), d, a);
                println!("{} + {}i", -b / (2f32 * a), sqrt(-d) / (2f32 * a));
                println!("{} - {}i", -b / (2f32 * a), sqrt(-d) / (2f32 * a));
            }
            d if d == 0f32 => {
                println!("The discriminant is strictly zero, the only solution is:");
                println!("^ Discriminant = {}^2 - 4 * {} * {} = {}", b, a, c, d);
                println!("^ X = ±{} - sqrt({}) / (2 * {})", b.abs(), d, a);
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
        println!("The solution is:");
        println!("^ {} * X^1 = {} * X^0", a, -b);
        println!("^ X = {} / {}", -b, a);
        println!("{}", -b / a);
    }

    pub fn try_solve(&self) {
        match self.get_degree() {
            2 => self.solve_second_degree(),
            1 => self.solve_first_degree(),
            0 => println!("Each real number is a solution."),
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

    #[test]
    fn push_term() {
        // let mut poly = Polynomial::new();
        // poly.push_term(Term::new(10, 0.0));
        // assert_eq!(poly.get_degree(), 0);

        // poly = Polynomial::new();
        // poly.push_term(Term::new(10, 1.0));
        // assert_eq!(poly.get_degree(), 10);
    }
}
