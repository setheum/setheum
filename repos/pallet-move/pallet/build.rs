use std::error::Error;
#[cfg(any(
    feature = "build-move-projects-for-test",
    feature = "runtime-benchmarks"
))]
use std::process::{Command, Output};

fn main() -> Result<(), Box<dyn Error>> {
    // Build move projects for the test purposes.
    #[cfg(any(
        feature = "build-move-projects-for-test",
        feature = "runtime-benchmarks"
    ))]
    build_move_projects()?;

    Ok(())
}

#[cfg(any(
    feature = "build-move-projects-for-test",
    feature = "runtime-benchmarks"
))]
fn build_move_projects() -> Result<(), Box<dyn Error>> {
    println!("cargo:warning=Building move projects in tests/assets folder");

    let smove_run = Command::new("bash")
        .args(["src/assets/move-projects/smove-build-all.sh"])
        .output()
        .expect("failed to execute script which builds necessary move modules");
    eval_smove_run(smove_run)?;

    println!("cargo:warning=Move projects built successfully");
    // Rerun in case Move source files are changed.
    println!("cargo:rerun-if-changed=tests/assets/move-projects");

    Ok(())
}

#[cfg(any(
    feature = "build-move-projects-for-test",
    feature = "runtime-benchmarks"
))]
fn eval_smove_run(smove_run: Output) -> Result<(), Box<dyn Error>> {
    if !smove_run.status.success() {
        let stderr = std::str::from_utf8(&smove_run.stderr)?;

        let e = Box::<dyn Error + Send + Sync>::from(stderr);
        Err(e)
    } else {
        Ok(())
    }
}
