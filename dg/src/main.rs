mod argh_ext;
use argh::FromArgs;

use std::process::exit;

/// dg: find dirty local Git repos with pending changes or unpushed content
#[derive(Debug, FromArgs)]
#[allow(clippy::struct_excessive_bools)]
struct AppArgs {
    /// include analysis for local branches
    #[argh(switch, short = 'b')]
    branches: bool,

    /// root path (default ".")
    #[argh(option, short = 'p')]
    path: Option<String>,
}

fn main() -> eyre::Result<()> {
    let args: AppArgs = argh_ext::from_env();
    let path = args.path.unwrap_or_else(|| ".".to_string());

    let res = dg::run(path.as_str(), args.branches);

    match res {
        Ok(ok) => {
            exit(i32::from(!ok));
        }
        Err(err) => {
            eprintln!("error: {err}");
            exit(1)
        }
    }
}
