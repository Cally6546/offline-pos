fn main() {
    let config = slint_build::CompilerConfiguration::new()
        // This tells the robot to look in the "ui" folder for any imports
        .with_include_paths(vec![std::path::PathBuf::from("ui")]);

    slint_build::compile_with_config("ui/Main.slint", config).expect("Slint build failed");
}
