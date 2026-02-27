use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    println!("cargo:rustc-link-search=native=libs");
    copy_dlls();
    tauri_build::build()
}
fn copy_dlls() {
    // 获取当前项目根目录 (src-tauri)
    let project_dir = env::current_dir().expect("Failed to get project dir");
    let libs_dir = project_dir.join("libs");

    // 获取输出目录 (例如 target/debug)
    // 注意：OUT_DIR 是 build.rs 专用的临时目录，我们需要向上找三级到 target/debug
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let target_dir = PathBuf::from(out_dir)
        .join("../../../")
        .canonicalize()
        .expect("Failed to get target dir");

    // 遍历 libs 文件夹，寻找所有 .dll 文件
    if let Ok(entries) = fs::read_dir(libs_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("dll") {
                let dest = target_dir.join(path.file_name().unwrap());

                // 执行拷贝：如果文件已存在且内容没变，fs::copy 会处理
                fs::copy(&path, &dest).expect("Failed to copy DLL to target directory");

                // 打印信息（在执行 cargo build -v 时可见）
                println!("cargo:warning=Auto-copied {} to {:?}", path.display(), dest);
            }
        }
    }
}
