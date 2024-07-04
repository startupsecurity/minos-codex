use clap::Parser;
use minos_codex::{create_scanner, MinosCodexError};
use rust_embed::RustEmbed;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "startup.security",
    about = "Detect and identify secrets in a string"
)]
struct Opts {
    /// The string to scan for secrets
    #[clap(name = "STRING")]
    input: String,
}

#[derive(RustEmbed)]
#[folder = "detections/"]
struct Assets;

fn main() -> Result<(), MinosCodexError> {
    let opts: Opts = Opts::parse();
    let mut scanner = create_scanner()?;

    let found_secrets = scanner.scan(&opts.input)?;

    if found_secrets.is_empty() {
        println!("No secrets found.");
    } else {
        println!("Found secrets:");
        for secret in found_secrets {
            println!("  Type: {}", secret.secret_type);
            println!("  Value: {}", secret.value);
            println!("  Position: {}:{}", secret.start, secret.end);
            println!();
        }
    }

    Ok(())
}
