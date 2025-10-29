use clap::Parser;

#[derive(Parser)]
#[command(name = "greeter")]
#[command(about = "A simple Cli tool to greet a person", long_about = "None")]
struct Greet {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = String::from("Hello"))]
    greeting: String,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    uppercase: bool,
}

fn main() {
    let args: Greet = Greet::parse();

    let mut message: String = format!("{}, {}!", args.greeting, args.name);

    if args.uppercase {
        message = message.to_uppercase();
    }

    println!("{}", message);
}
