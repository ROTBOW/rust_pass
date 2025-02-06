use clap::{Parser, Subcommand};

/// simple password manager
#[derive(Parser, Debug)]
#[command(version = "0.0.1", about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// saying hi
    Greet {
        /// name of the person
        #[arg(short, long)]
        name: String,

        /// count times to say it
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },

    /// Check if your logged in
    Check {},


}


/// COMMANDS ///
fn greet(name: String, count: u8) -> () {
    for _ in 0..count {
        println!("hey {}!", name)
    }
}


fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Greet { name, count } => {
            greet(name, count)
        }
        Commands::Check {} => {
            println!("woah")
        }
    }
}
