use clap::{Parser, Subcommand};

use geometry::{Area, Circle, Rectangle, Square, Triangle};

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
    Square { side: f32 },

    /// The Circle shape
    #[command(arg_required_else_help = true)]
    Circle { radius: f32 },

    /// The Rectangle shape
    #[command(arg_required_else_help = true)]
    Rectangle { width: f32, height: f32 },

    /// The Triangle shape
    #[command(arg_required_else_help = true)]
    Triangle { width: f32, height: f32 },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Square { side } => {
            println!("Area of the square: {:?} units^2.", Square { side }.area())
        }
        Commands::Circle { radius } => {
            println!(
                "Area of the square:car {:?} units^2.",
                Circle { radius }.area()
            )
        }
        Commands::Rectangle { width, height } => {
            println!(
                "Area of the square: {:?} units^2.",
                Rectangle { width, height }.area()
            )
        }
        Commands::Triangle { width, height } => {
            println!(
                "Area of the square: {:?} units^2.",
                Triangle { width, height }.area()
            )
        }
    }
}
