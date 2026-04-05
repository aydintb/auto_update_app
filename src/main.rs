use self_updater_helper::{run_update, UpdaterConfig};

fn hello_world() {
    println!("Hello, world!");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = UpdaterConfig {
        owner: "aydintb".into(),
        repo: "auto_update_app".into(),
        bin_name: "auto_update_app".into(),
        current_version: env!("CARGO_PKG_VERSION").into(),
        ..Default::default()
    };

    run_update(&config)?;

    hello_world();

    println!("App running...");
    Ok(())
}
