use std::path::PathBuf;

use clap::error::ErrorKind;
use clap::{Args, Error, Parser, Subcommand};

use booru_rs::manager::Engine;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "image-dataset-fetcher")]
#[command(bin_name = "image-dataset-fetcher")]
#[command(version = env ! ("CARGO_PKG_VERSION"))]
// TODO pass git commit
#[command(long_version = format ! (r#"v{version}
 commit: {commit}"#, version = env ! ("CARGO_PKG_VERSION"), commit = "TODO"))]
pub struct Cli {
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // /// Verbosity log debug
    // pub verbose: u8,
    #[arg(short, long)]
    /// Verbosity log debug
    pub proxy: Option<String>,

    // #[arg(long)]
    // /// Write log to file
    // pub log_file: Option<PathBuf>,
    #[arg(short, long)]
    /// Path to store fetched dataset
    pub target: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Fetch from booru
    Booru(BooruCommand),
}

#[derive(Debug, Args)]
#[command(author, version, about, long_about = None)]
pub struct BooruCommand {
    /// Booru engine
    #[arg(short, long, default_value_t = Engine::Danbooru, value_parser = parse_engine)]
    pub engine: Engine,

    /// Booru url
    #[arg(short, long)]
    pub url: Option<String>,

    /// Query.
    #[arg(short, long)]
    pub query: Option<String>,

    /// Limit
    #[arg(short, long, default_value_t = 100)]
    pub limit: u32,

    /// Num pages for fetch. 0 - infinity
    #[arg(short, long, default_value_t = 1)]
    pub pages: u32,
}

/// Parse a single key-value pair
fn parse_engine(s: &str) -> Result<Engine, Error> {
    Engine::try_from(s).map_err(|_| Error::new(ErrorKind::InvalidValue))
}

#[cfg(test)]
mod tests {
    use super::Commands::*;
    use super::*;

    #[test]
    fn test_booru_cli() {
        let args =
            "image-dataset-fetcher --proxy foobar -t /tmp/ booru -e danbooru -q foo -p 2 -l 50"
                .split(' ');
        let cli = Cli::try_parse_from(args).unwrap();
        // assert_eq!(cli.verbose, 0);
        // assert_eq!(cli.log_file, None);
        assert_eq!(cli.proxy, Some("foobar".to_string()));

        match cli.command {
            Booru(server) => {
                assert_eq!(server.engine, Engine::Danbooru);
                assert_eq!(server.query, Some("foo".to_string()));
                assert_eq!(server.pages, 2);
                assert_eq!(server.limit, 50);
            }
        }
    }
}
