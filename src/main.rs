use std::str::FromStr;

fn main() {
    use std::env;
    use std::fs;

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    match fs::read_to_string(&args[1]) {
        Ok(contents) => {
            let partial = http_message::PartialHttpRequest::from_str(&contents);

            println!("{partial:#?}");
        }
        Err(e) => eprintln!("Error reading file {}: {}", args[1], e),
    }
}
