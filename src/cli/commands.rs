// Subcommand definitions for the CLI.

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Analyze a file or directory and print a project summary.
    Analyse {
        #[arg(help = "Path to the file or directory to analyze")]
        path: String,

        #[arg(short, long, help = "Show detailed analysis output")]
        verbose: bool,

        #[arg(long, help = "Print a directory tree and exit")]
        tree: bool,
    },
    /// Show version information.
    Version,
}
