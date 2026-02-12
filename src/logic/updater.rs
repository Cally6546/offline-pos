use self_update::backends::github::Update;

pub fn check_for_updates() -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking for updates...");

    let status = Update::configure()
        .repo_owner("Cally6546") // Your GitHub name
        .repo_name("offline-pos")          // Your Project name
        .bin_name("offline-pos")           // The name of the .exe or binary
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION")) // This reads your Cargo.toml version
        .build()?
        .update()?;

    println!("Update status: v{}", status.version());
    Ok(())
}
