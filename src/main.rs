fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = (|| -> Result<(), String> {
        if args.len() == 2 && args[1] == "--version" {
            println!("QRUNAP2 {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        if args.len() < 3 || args.len() > 4 || !["check", "run"].contains(&args[1].as_str()) {
            return Err(
                "Usage: qrunap2 check FILE | qrunap2 run FILE [SEED] | qrunap2 --version".into(),
            );
        }
        let source = std::fs::read_to_string(&args[2]).map_err(|e| e.to_string())?;
        let model = qrunap2::parse(&source)?;
        if args[1] == "check" {
            println!("OK: {} stations", model.stations.len());
        } else {
            let seed = args
                .get(3)
                .map(|s| s.parse::<u32>().map_err(|_| "Invalid seed".to_string()))
                .transpose()?
                .unwrap_or(42);
            println!(
                "{}",
                serde_json::to_string_pretty(&qrunap2::simulate(&model, seed)?)
                    .map_err(|e| e.to_string())?
            );
        }
        Ok(())
    })();
    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
