# Shell Snippets

### Estimate Gas for Module `CarWash`

Requirement: Before publishing the module for user _Bob_.

```sh
smove node rpc estimate-gas-publish-module -a 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty -m build/car-wash-example/bytecode_modules/CarWash.mv
```

### Estimate Gas for Script `initial_coin_minting`

Requirement: After publishing the module for user _Bob_.
```sh
smove node rpc estimate-gas-execute-script -s build/car-wash-example/script_transactions/initial_coin_minting.mvt
```

### Estimate Gas for Script `register_new_user`

Requirement: After publishing the module for an arbitrary user.
```sh
smove node rpc estimate-gas-execute-script -s build/car-wash-example/script_transactions/register_new_user.mvt
```

### Estimate Gas for Script `buy_coin`

Requirement: After registering a user with same user. Works for buying one coin.
```sh
smove node rpc estimate-gas-execute-script -s build/car-wash-example/script_transactions/buy_coin.mvt
```

### Estimate Gas for Script `wash_car`

Requirement: unknown.
```sh
smove node rpc estimate-gas-execute-script -s build/car-wash-example/script_transactions/wash_car.mvt
```
