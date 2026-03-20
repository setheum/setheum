use anyhow::Result;
use clap::Parser;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use move_binary_format::access::ModuleAccess;
use move_binary_format::CompiledModule;
use move_command_line_common::files::MOVE_COMPILED_EXTENSION;
use move_core_types::language_storage::ModuleId;
use move_vm_backend_common::types::ModuleBundle;

use crate::run_context::RunContext;
use crate::run_move_cli;

/// Package command bundles modules into packages.
#[derive(Parser, Debug, Clone)]
#[clap(about = "setheum-move bundle")]
pub struct Bundle {
    #[clap(
        short,
        long,
        help = "Bundle name. By default it is the same as the name of the package."
    )]
    name: Option<PathBuf>,

    // Modules are taken from the <PROJECT_PATH>/build/<PROJECT_NAME>/bytecode_modules directory.
    // The names are case-insensitive and can be specified with an extension.mv or without it.
    #[clap(
        help = "Names of modules to exclude from the bundling process.",
        long = "modules_exclude",
        multiple_values = true
    )]
    modules_exclude: Vec<String>,
}

impl Bundle {
    /// Executes the command.
    pub fn execute(&mut self, ctx: &RunContext) -> Result<()> {
        // Build all move modules
        run_move_cli::execute_build(ctx)?;

        // Add extension to filtered modules to make comparsion more easy.
        let mut modules_exclude_ext = Vec::with_capacity(self.modules_exclude.len());
        while let Some(module) = self.modules_exclude.pop() {
            modules_exclude_ext.push(OsString::from(module + "." + MOVE_COMPILED_EXTENSION))
        }

        // Get all bytecode modules (without external dependecies and without excluded ones)
        let module_paths = ctx
            .get_bytecode_modules()?
            .into_iter()
            .filter(|module| {
                !modules_exclude_ext.contains(&module.file_name().unwrap().to_os_string())
            })
            .collect::<Vec<PathBuf>>();

        let modules: Vec<Vec<u8>> = module_paths.into_iter().flat_map(std::fs::read).collect();

        let sorted_modules = sort_modules(modules);
        let bundle = ModuleBundle::new(sorted_modules);

        // Path to the output file
        let bundle_name = match self.name {
            Some(ref name) => name.clone(),
            _ => PathBuf::from(ctx.manifest()?.package.name.as_str()),
        };
        let output_file_path = ctx.bundle_output_path(&bundle_name)?;
        if output_file_path.exists() {
            fs::remove_file(&output_file_path)?;
        }

        fs::write(&output_file_path, bundle.encode()?)?;

        println!(
            "Modules are bundled under: {}",
            output_file_path.canonicalize()?.display()
        );

        Ok(())
    }
}

/// Returns module bytecode in sorted order according to dependecies.
fn sort_modules(modules: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut map = modules
        .into_iter()
        .map(|bytecode| {
            let module = CompiledModule::deserialize(&bytecode).unwrap();
            (module.self_id(), (bytecode, module))
        })
        .collect::<BTreeMap<_, _>>();

    let mut order = vec![];
    for id in map.keys() {
        sort_by_deps(&map, &mut order, id.clone());
    }

    let mut result = vec![];
    for id in order {
        let (bytecode, _) = map.remove(&id).unwrap();
        result.push(bytecode)
    }

    result
}

/// Recursively sorts dependencies for compiled Move modules.
fn sort_by_deps(
    map: &BTreeMap<ModuleId, (Vec<u8>, CompiledModule)>,
    order: &mut Vec<ModuleId>,
    id: ModuleId,
) {
    if order.contains(&id) {
        return;
    }

    let (_, compiled_module) = &map.get(&id).unwrap();
    for dep in compiled_module.immediate_dependencies() {
        // Only consider deps which are actually in this package. Deps for outside
        // packages are considered fine because of package deployment order.
        if map.contains_key(&dep) {
            sort_by_deps(map, order, dep);
        }
    }
    order.push(id)
}
