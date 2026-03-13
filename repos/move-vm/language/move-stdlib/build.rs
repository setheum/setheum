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
    let run_build_stdlib_script = Command::new("bash")
        .args(["build_stdlid_with_smove.sh"])
        .output()
        .expect("failed to execute process");
    if !run_build_stdlib_script.status.success() {
        let stderr = std::str::from_utf8(&run_build_stdlib_script.stderr)?;

        let e = Box::<dyn Error + Send + Sync>::from(stderr);
        return Err(e);
    }

    // Rerun in case Move source files are changed.
    println!("cargo:rerun-if-changed=sources/");
    Ok(())
}
