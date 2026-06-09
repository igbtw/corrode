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

        #[arg(
            long,
            help = "Export report as JSON",
            conflicts_with_all = &["tree", "markdown"]
        )]
        json: bool,

        #[arg(
            long,
            help = "Export report as Markdown",
            conflicts_with_all = &["tree", "json"]
        )]
        markdown: bool,
    },
    /// Show version information.
    Version,
}
