use std::error::Error;
#[cfg(feature = "stdlib-bytecode")]
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "stdlib-bytecode")]
    build_stdlib_with_smove()?;

    Ok(())
}

#[cfg(feature = "stdlib-bytecode")]
fn build_stdlib_with_smove() -> Result<(), Box<dyn Error>> {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    
    // Path to setheum-move binary
    let mut setheum_move_path = manifest_dir.clone();
    setheum_move_path.pop(); // move-stdlib
    setheum_move_path.pop(); // language
    setheum_move_path.pop(); // move-vm
    setheum_move_path.push("setheum/target/debug/setheum-move");

    // Path to the shell script
    let script_path = manifest_dir.join("build_stdlid_with_smove.sh");

    println!("cargo:warning=Manifest Dir: {:?}", manifest_dir);
    println!("cargo:warning=Setheum-move Path: {:?}", setheum_move_path);
    println!("cargo:warning=Script Path: {:?}", script_path);

    if !setheum_move_path.exists() {
        return Err(format!("setheum-move binary not found at {:?}", setheum_move_path).into());
    }
    if !script_path.exists() {
        return Err(format!("build script not found at {:?}", script_path).into());
    }

    let run_build_stdlib_script = Command::new("bash")
        .args([script_path.to_string_lossy().as_ref(), setheum_move_path.to_string_lossy().as_ref()])
        .output()
        .expect("failed to execute process");
    if !run_build_stdlib_script.status.success() {
        let stderr = std::str::from_utf8(&run_build_stdlib_script.stderr)?;
        println!("cargo:warning=Build script stderr: {}", stderr);
        return Err(stderr.into());
    }

    // Rerun in case Move source files are changed.
    println!("cargo:rerun-if-changed=sources/");
    Ok(())
}
