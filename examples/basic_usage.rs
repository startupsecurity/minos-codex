use minos_codex::{create_scanner, FoundSecret, MinosCodexError};

fn main() {
    match run() {
        Ok(_) => println!("Scan completed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn run() -> Result<(), MinosCodexError> {
    let mut scanner = create_scanner("detections")?;

    let input = "My email is example@email.com and my AWS access key is AKIAIOSFODNN7EXAMPLE";

    let found_secrets = scanner.scan(input)?;

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
