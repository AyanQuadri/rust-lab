use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "greeter")]
#[command(about = "A simple Cli tool to greet a person", long_about = "None")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello {
        #[arg(short, long)]
        name: String,
    },

    Goodbye {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Hello { name } => {
            println!("Hello, {}!", name);
        }
        Commands::Goodbye { name } => {
            println!("Hello, {}!", name);
        }
    }
}
