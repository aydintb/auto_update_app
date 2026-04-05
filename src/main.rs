use self_update::cargo_crate_version;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

fn hello_world() {
    println!("Hello, world!");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Current version: {}", cargo_crate_version!());

    let exe_path = std::env::current_exe()?; // capture before update (Linux marks replaced binary as deleted)

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

        // restart after update using exec (replaces current process, no stdio race)
        #[cfg(unix)]
        std::process::Command::new(&exe_path).exec();
        #[cfg(windows)]
        {
            std::process::Command::new(&exe_path).spawn()?;
            std::process::exit(0);
        }
    } else {
        println!("Already up to date.");
    }

    hello_world();

    // --- your app logic ---
    println!("App running...");
    Ok(())
}

