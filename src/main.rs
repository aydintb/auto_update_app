use self_update::cargo_crate_version;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Current version: {}", cargo_crate_version!());

    // --- updater ---
    let status = self_update::backends::github::Update::configure()
        .repo_owner("aydintb")                 // your GitHub username
        .repo_name("auto_update_app")          // your repo
        .bin_name("auto_update_app")           // binary name
        .target(self_update::get_target())     // auto detect OS
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    // --- result ---
    if status.updated() {
        println!("Updated to version: {}", status.version());

        // restart after update
        let exe = std::env::current_exe()?;
        std::process::Command::new(exe).spawn()?;
        std::process::exit(0);
    } else {
        println!("Already up to date.");
    }

    // --- your app logic ---
    println!("App running...");
    Ok(())
}

