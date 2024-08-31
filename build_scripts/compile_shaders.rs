use std::path::Path;
use std::process::Command;

pub fn compile_metal_shaders() {
    println!("cargo:rerun-if-changed=src/metal_shaders");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let shader_dir = Path::new("src/metal_shaders");

    // Compile .metal files to .air files
    for entry in std::fs::read_dir(shader_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().unwrap_or_default() == "metal" {
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let air_file = format!("{}/{}.air", out_dir, file_stem);

            println!("Compiling shader: {:?}", path);
            let status = Command::new("xcrun")
                .args([
                    "-sdk",
                    "macosx",
                    "metal",
                    "-c",
                    path.to_str().unwrap(),
                    "-o",
                    &air_file,
                ])
                .status()
                .expect("Failed to compile shader");

            if !status.success() {
                panic!("Failed to compile shader: {:?}", path);
            }
        }
    }

    // Combine .air files into a single .metallib file
    let metallib_file = format!("{}/shaders.metallib", out_dir);
    let mut command = Command::new("xcrun");
    command.args(["-sdk", "macosx", "metallib"]);

    for entry in std::fs::read_dir(&out_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().unwrap_or_default() == "air" {
            command.arg(path);
        }
    }

    command.args(["-o", &metallib_file]);
    let status = command.status().expect("Failed to create metallib");

    if !status.success() {
        panic!("Failed to create metallib");
    }

    println!("cargo:rustc-env=METAL_SHADER_LIB={}", metallib_file);
}
