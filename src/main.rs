use calculator::{Calculator, Error};

mod calculator;

fn main() -> Result<(), Error> {
    loop {
        println!("\nType an expression for evaluation..\n");

        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let tokens = calculator::Calculator::parse(input);

                if tokens.is_err() {
                    println!("{:?}", tokens.err().unwrap());
                    continue;
                }

                let expression = calculator::Calculator::expression(tokens?);

                if let Some(result) = Calculator::evaluate(expression) {
                    println!("{}", result)
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
