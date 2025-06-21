// Build script to embed Windows application icon
// For Windows targets, this will:
// 1. Ensure there is an ICO file generated from assets/icon.png (if not already present).
// 2. Generate a temporary RC file referencing that ICO.
// 3. Use `embed-resource` to compile it into a `.res` linked into final exe, so the file
//    icon is shown in Explorer / taskbar.
// On non-Windows targets, this script does nothing.

#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    // 指定 ico 路径，Cargo 会在文件变化时重新运行 build.rs
    println!("cargo:rerun-if-changed=assets/icon.ico");

    // 编译并嵌入图标

    if let Err(e) = winres::WindowsResource::new()
        .set_icon("assets/icon.ico")
        .compile()
    {
        println!("cargo:warning=winres compile failed: {e}");
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {}
