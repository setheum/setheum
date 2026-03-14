# Shell Snippets

### Estimate Gas for Module `Dorm`

Requirement: Before publishing the module for user _Bob_.

```sh
smove node rpc estimate-gas-publish-module -a 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty -m build/multiple-signers/bytecode_modules/Dorm.mv
```

### Estimate Gas for Script `init_module`

Requirement: After publishing the module for user _Bob_.
```sh
smove node rpc estimate-gas-execute-script -s build/multiple-signers/script_transactions/init_module.mvt
```

### Estimate Gas for Script `rent_apartment`

Requirement: After publishing the module for user _Bob_ and initializing the module with `init_module`.
```sh
smove node rpc estimate-gas-execute-script -s build/multiple-signers/script_transactions/rent_apartment.mvt
```
