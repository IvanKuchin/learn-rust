use minigrep;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = minigrep::Config::build(&args).unwrap_or_else(|err_message| {
        println!("Problem parsing arguments: {}", err_message);
        std::process::exit(1);
    });
    println!("Searching for {}", config.query());
    println!("In file {}", config.filename());

    if let Err(e) = minigrep::run(&config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}

