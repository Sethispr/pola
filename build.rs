fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::Build::new();
        res.set_icon("icon.ico");
        res.compile().expect("Failed to compile resources");
    }
}
