mod math;
mod parser;

use std::{env, error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let input = match args.as_slice() {
        [_, arg] => arg.clone(),
        [_] => {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            buffer.trim().to_string()
        }
        _ => {
            eprintln!("Usage: computor <eq> or echo <eq> | computor");
            return Err("Wrong number of arguments!".into());
        }
    };

    let mut poly = parser::parse(&input)?;

    println!("Reduced form: {}", poly.print_reduced_form());
    println!("Free form: {}", poly.print_free_form());
    println!("Polynomial degree: {}", poly.get_degree());
    poly.try_solve();
    Ok(())
}
