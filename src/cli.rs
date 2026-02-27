use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pascal", about = "Python monorepo manager", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Bootstrap a new Pascal workspace
    Init {
        /// Name of the workspace (defaults to directory name)
        name: Option<String>,

        /// Python version to use
        #[arg(long, default_value = "3.12")]
        python: String,
    },

    /// Scaffold a new package or app
    Create {
        #[command(subcommand)]
        kind: CreateKind,
    },

    /// Add a workspace package as a dependency of an app or package
    Add {
        /// Name of the package to add
        package: String,

        /// Target app or package to add the dependency to
        #[arg(long)]
        to: String,
    },

    /// Pretty-print workspace overview
    Info,

    /// Print or visualize the dependency tree
    Deps {
        /// Show a textual graph instead of a list
        #[arg(long)]
        graph: bool,
    },

    /// Validate workspace (missing deps, circular refs)
    Check,

    /// Show packages/apps changed since a git ref or tag
    Diff {
        /// Git ref (commit, tag, or branch) to compare against
        #[arg(long)]
        since: Option<String>,
    },

    /// Run tests for packages/apps
    Test {
        /// Only test bricks with changes since last git tag
        #[arg(long)]
        changed: bool,

        /// Name of specific package or app to test
        name: Option<String>,

        /// Extra arguments to pass to pytest
        #[arg(last = true)]
        extra: Vec<String>,
    },

    /// Build an app with uv build
    Build {
        /// Name of the app to build
        app: String,
    },

    /// Run an app with uv run
    Run {
        /// Name of the app to run
        app: String,

        /// Extra arguments passed to the app
        #[arg(last = true)]
        extra: Vec<String>,
    },

    /// Regenerate root pyproject.toml and uv workspace config
    Sync,
}

#[derive(Subcommand)]
pub enum CreateKind {
    /// Scaffold a new reusable package
    Package {
        /// Name of the package (snake_case recommended)
        name: String,
    },
    /// Scaffold a new deployable app
    App {
        /// Name of the app (snake_case recommended)
        name: String,
    },
}
