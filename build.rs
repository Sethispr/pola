fn main() {
    if cfg!(target_os = "windows") {
        let icon_path = r"C:\Users\Documents\pola\assets\pola.ico";
        // tell Cargo to re-run the build script if the icon changes
        println!("cargo:rerun-if-changed={}", icon_path);

        let mut res = winres::WindowsResource::new();
        res.set_icon(icon_path);

        // add extra metadata
        res.set("FileDescription", "Silent Assassin TUI to efficiently search skins, check values and find owners!");
        res.set("ProductName", "pola");
        res.set("LegalCopyright", "Copyright (C) 2025 sethispr");

        // compile the resource file with error handling
        if let Err(e) = res.compile() {
            panic!("Failed to compile resources: {}", e);
        }
    }
}
