/// Configures Windows-specific resources for the application.
///
/// This function checks if the target operating system is Windows. If so, it:
/// 
/// - Sets the path to the icon file.
/// - Instructs Cargo to re-run the build script if the icon file changes.
/// - Creates a new Windows resource and sets the icon.
/// - Adds extra metadata (file description, product name, and legal copyright).
/// - Attempts to compile the resource file, panicking if it fails.
///
/// # Panics
///
/// Panics if the resource compilation fails.
fn main() {
    if cfg!(target_os = "windows") {
        let icon_path = r"C:\Users\YourUsername\YourDirectory\pola\assets\pola.ico";
        // Tell Cargo to re-run the build script if the icon changes.
        println!("cargo:rerun-if-changed={}", icon_path);

        let mut res = winres::WindowsResource::new();
        res.set_icon(icon_path);

        // Add extra metadata.
        res.set(
            "FileDescription",
            "Silent Assassin TUI to efficiently search skins, check values and find owners!",
        );
        res.set("ProductName", "pola");
        res.set("LegalCopyright", "Copyright (C) 2025 sethispr");

        // Compile the resource file, panicking on error.
        if let Err(e) = res.compile() {
            panic!("Failed to compile resources: {}", e);
        }
    }
}
