// use minigrep;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = minigrep::Config::build(&args).unwrap_or_else(|err_message| {
        eprintln!("Problem parsing arguments: {}", err_message);
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

