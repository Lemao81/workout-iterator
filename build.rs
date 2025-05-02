use winres::WindowsResource;

const ICON_PATH: &str = "./resources/icon.ico";

fn main() {
    println!("cargo::rerun-if-changed={ICON_PATH}");

    set_icon();
}

fn set_icon() {
    if cfg!(target_os = "windows") {
        let mut res = WindowsResource::new();
        res.set_icon(ICON_PATH);
        res.compile().unwrap();
    }
}
