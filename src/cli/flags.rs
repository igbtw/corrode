// CLI argument parsing. Uses clap derive macros to generate
// the parser from struct/enum definitions at compile time.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rsfactai", version, about = "Codebase analysis engine")]
pub struct Cli {
    /// Wrapped in Option so global flags (--license) can be used
    // without a subcommand — clap doesn't require a subcommand
    // when Option is used.
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(short = 'L', long, help = "Print license information")]
    pub license: bool,
}

// Analyse runs the full pipeline. Version just prints the version.
#[derive(Subcommand)]
pub enum Command {
    /// Analyze a file or directory and print a project summary.
    Analyse {
        #[arg(help = "Path to the file or directory to analyze")]
        path: String,

        #[arg(short, long, help = "Show detailed analysis output")]
        verbose: bool,

        // --tree prints a directory tree (├── └──) and exits.
        #[arg(long, help = "Print a directory tree and exit")]
        tree: bool,
    },
    /// Show version information.
    Version,
}

// Delegates to clap's built-in parser which reads env::args()
// and handles --help/--version/errors internally.
pub fn parse_args() -> Cli {
    Cli::parse()
}
