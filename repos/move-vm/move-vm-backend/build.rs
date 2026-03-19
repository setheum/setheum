use std::error::Error;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    // Build move projects for the test purposes.
    #[cfg(feature = "build-move-projects-for-test")]
    build_move_projects()?;

    Ok(())
}

#[allow(dead_code)]
fn build_move_projects() -> Result<(), Box<dyn Error>> {
    println!("cargo:warning=Building move projects in tests/assets folder");

    let smove_run = Command::new("bash")
        .args(["tests/assets/move-projects/smove-build-all.sh"])
        .output()
        .expect("failed to execute script which builds necessary move modules");

    if !smove_run.status.success() {
        let stderr = std::str::from_utf8(&smove_run.stderr)?;

        let e = Box::<dyn Error + Send + Sync>::from(stderr);
        return Err(e);
    }

    println!("cargo:warning=Move projects built successfully");
    // Rerun in case Move source files are changed.
    println!("cargo:rerun-if-changed=tests/assets/move-projects");

    Ok(())
}
