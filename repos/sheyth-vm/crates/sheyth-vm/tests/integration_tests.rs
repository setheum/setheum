#[test]
fn initialize_default_engine_with_dynamic_paging() {
    let _ = env_logger::try_init();
    let mut config = sheyth_vm::Config::from_env().unwrap();
    config.set_worker_count(1);
    config.set_allow_dynamic_paging(true);
    sheyth_vm::Engine::new(&config).unwrap();
}

#[test]
fn initialize_default_engine_without_dynamic_paging() {
    let _ = env_logger::try_init();
    let mut config = sheyth_vm::Config::from_env().unwrap();
    config.set_allow_dynamic_paging(false);
    sheyth_vm::Engine::new(&config).unwrap();
}
