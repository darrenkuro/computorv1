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
            eprintln!("\x1b[33m[ Usage ] computor <eq> or echo <eq> | computor\x1b[0m");
            return Err("Wrong number of arguments!".into());
        }
    };

    let mut poly = parser::parse(&input)?;

    println!("{:#?}", poly);

    println!("[ POLYNOMIAL DEGREE ] {}", poly.get_degree());
    println!("[    REDUCED FORM   ] {}", poly.print_reduced_form());
    println!(
        "\x1b[33m[  FREE ENTRY FORM  ] {}\x1b[0m",
        poly.print_free_form()
    );
    poly.try_solve();
    Ok(())
}
