mod math;
mod parser;

use std::env;
use std::io;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let input = match args.len() {
        1 => {
            // Read from stdin
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .map_err(|e| format!("Failed to read stdin: {}", e))?;
            buffer.trim().to_string()
        }
        2 => args[1].clone(),
        _ => return Err("Wrong number of arguments!".to_string()),
    };

    match parser::parse(&input) {
        Ok(mut poly) => {
            println!("Reduced form: {}", poly.print_form());
            println!("Polynomial degree: {}", poly.get_degree());
            poly.try_solve();
            Ok(())
        }
        Err(e) => Err(e),
    }
}
