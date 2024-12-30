use rlox::Lox;

fn main() -> Result<(), rlox::Error> {
    let args: Vec<String> = std::env::args().collect();

    #[allow(clippy::comparison_chain)]
    if args.len() > 2 {
        println!("Usage: {} [script]", args[0]);
        std::process::exit(64);
    } else if args.len() == 2 {
        todo!()
    } else {
        Lox::run_prompt()
    }
}
