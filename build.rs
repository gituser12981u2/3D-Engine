use build_scripts::compile_shaders::compile_metal_shaders;
mod build_scripts;

fn main() {
    compile_metal_shaders();

    println!("cargo:rerun-if-changed=build-scripts");
}
