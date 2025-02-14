use clap::{Parser, Subcommand};

const PI: f64 = 3.1415;

/// A CLI to calculate geometrical shapes properties
#[derive(Debug, Parser)]
#[command(name = "geometry")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// The Square shape
    #[command(arg_required_else_help = true)]
    Square { side: f64 },

    /// The Circle shape
    #[command(arg_required_else_help = true)]
    Circle { radius: f64 },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Square { side } => {
            println!("Square of area {:?}", side * side);
        }
        Commands::Circle { radius } => {
            println!("Circle of area {:?}", PI * radius.powf(2.0))
        }
    }
}
