use anyhow::{Error, Result};
use move_cli::Move as MoveCliArgs;
use move_command_line_common::files::{extension_equals, find_filenames, MOVE_COMPILED_EXTENSION};
use move_core_types::errmap::ErrorMapping;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_package::compilation::package_layout::CompiledPackageLayout;
use move_package::source_package::parsed_manifest::SourceManifest;
use move_package::source_package::{layout, manifest_parser};
use move_stdlib::natives::all_natives;
use move_vm_backend_common::gas_schedule::{INSTRUCTION_COST_TABLE, NATIVE_COST_PARAMS};
use move_vm_runtime::native_functions::NativeFunctionTable;
use move_vm_test_utils::gas_schedule::CostTable;
use std::fs;
use std::path::{Path, PathBuf};

/// Move compilation related data.
pub struct RunContext {
    /// Project directory.
    pub project_root_dir: PathBuf,
    /// `move_cli` arguments.
    pub move_args: MoveCliArgs,
    /// Error descriptions.
    pub error_descriptions: ErrorMapping,
    /// Native functions.
    pub natives: NativeFunctionTable,
    /// Cost table.
    pub cost_table: CostTable,
    /// `Move.toml` contents for the current folder.
    manifest: Option<SourceManifest>,
}

impl RunContext {
    /// Create a new instance.
    pub fn new(project_root_dir: PathBuf, move_args: MoveCliArgs) -> Result<Self> {
        let cost_table = INSTRUCTION_COST_TABLE.clone();
        let natives = all_natives(CORE_CODE_ADDRESS, NATIVE_COST_PARAMS.clone());
        let error_descriptions = bcs::from_bytes(move_stdlib::doc::error_descriptions())?;

        let manifest_path = project_root_dir.join(layout::SourcePackageLayout::Manifest.path());
        let manifest = manifest_parser::parse_move_manifest_from_file(&manifest_path).ok();

        Ok(Self {
            project_root_dir,
            move_args,
            manifest,
            error_descriptions,
            natives,
            cost_table,
        })
    }

    /// Get manifest for the current folder.
    pub fn manifest(&self) -> Result<&SourceManifest, Error> {
        self.manifest.as_ref().ok_or(Error::msg(format!(
            "Manifest file not found at {}",
            self.project_root_dir.display()
        )))
    }

    /// Path where bundles are generated.
    pub fn bundle_output_path(&self, bundle_name: &impl AsRef<Path>) -> Result<PathBuf, Error> {
        let package_name = self.manifest()?.package.name.as_str();

        // Create directory "<PACKAGE_PATH>/build/<PACKAGE_NAME>/bundles/"
        let dir = self
            .project_root_dir
            .join(CompiledPackageLayout::Root.path())
            .join(package_name)
            .join("bundles");

        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }

        Ok(dir.join(bundle_name).with_extension("mvb"))
    }

    /// Path where script transactions are generated.
    pub fn script_tx_output_path(&self, tx_name: &impl AsRef<Path>) -> Result<PathBuf, Error> {
        let package_name = self.manifest()?.package.name.as_str();

        // Create directory "<PACKAGE_PATH>/build/<PACKAGE_NAME>/script_transactions/"
        let dir = self
            .project_root_dir
            .join(CompiledPackageLayout::Root.path())
            .join(package_name)
            .join("script_transactions");

        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }

        Ok(dir.join(tx_name).with_extension("mvt"))
    }

    /// Get paths for all compiled modules without dependencies.
    pub fn get_bytecode_modules(&self) -> Result<Vec<PathBuf>> {
        let bytecode_modules_dir = CompiledPackageLayout::CompiledModules.path().as_os_str();

        let files = find_filenames(&[&self.project_root_dir], |path| {
            // Use unwrap: if the parent exists - then it has to have a name.
            let parent_dir = path.parent().map(|path| path.file_name().unwrap());

            // If the .mv file is under "bytecode_modules" dir - bundle it.
            extension_equals(path, MOVE_COMPILED_EXTENSION)
                && parent_dir == Some(bytecode_modules_dir)
        })?;

        Ok(files.into_iter().map(PathBuf::from).collect())
    }
}
