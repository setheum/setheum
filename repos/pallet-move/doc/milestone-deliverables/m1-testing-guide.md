_INFO - This document is deprecated and no longer maintained - use it for informational purposes only.
The actual testing guide can be found in [tech-guide document](../tech_guide.md)._

# Testing guide (deprecated)
Welcome to the Testing Guide! This comprehensive guide is designed to equip you with the knowledge and tools necessary to understand and execute all prepared tests.

## Testing approach
There are many testing approaches used in the software industry, each with its focus and objectives. These testing approaches are used during different phases of the software development lifecycle to ensure the quality and reliability of the software. 

Test layers refer to the different levels or stages of testing that are performed to ensure the quality and reliability of the software. During the project, we will use the following layers: 
- Source analysis.
- Unit testing.
- Integration testing.
- System (black-box) testing.
- Acceptance testing.
- Code coverage.

The testing approach will differ depending on where new or modified code will appear. Each thing added in the `pallet-move` repository (and any other crates inside this repository like RPC crates or runtime API) will be tested using the full testing set, ensuring each test layer is covered. It will ensure that produced output is correct and expected.

`pallet-move` is a central place which gets all the work together as it references the Move fork crate, contains the API and RPC and is incorporated directly in the Substrate node executable. It means there is a need to make some changes also to external repositories in order to work with this crate - those changes are also subject to be covered by the tests. As we are considering here external repositories there is a need to fit into their testing structure and organization.

It's desirable to reach at least 70-80% of test coverage when summing all layers for the `pallet-move` repository. Instructions on how to run code coverage and test coverage tools are described in the [Code coverage](https://doc.rust-lang.org/rustc/instrument-coverage.html) Rust documentation.

All changes done to the Move Virtual Machine or Move language will be covered by the test set compatible with other functionalities already present in that module, taking into account the rules given in further chapters of this document. It means that if we introduce some new functions they will be covered by the same test suite the other functions are. Move smart contracts intended to be production-use will be tested using unit tests. 

Changes introduced to the Substrate node will be covered by the test set compatible with other functionalities already present in the node unless they will require some new mechanisms that aren't used in the node yet. In fact, a fork of the template node is used only for testing purposes to provide a system (black-box) testing framework, and it shouldn't contain many functional changes (besides changes that incorporate pallet and allows to call it functions and receive results - and that will be tested). The software design ensures that the MoveVM pallet is independent enough to be tested separately.

## Code analysis
The first and very basic way of assuring code correctness is to use code analysis tools. Rust provides a set of tools that can be used to analyze the code and find potential issues. The team will use `clippy` linter to check the code for potential issues and `rustfmt` to format the code according to the Rust style guidelines.

There is a possibility to run `clippy` and `rustfmt` using the following commands:
```bash
cargo clippy
cargo fmt
```

Both tools will be run on each commit to the repository, so it's not necessary to run them manually. The CI will fail if any of the tools will report any issues.

## Unit tests
Unit testing is the foundational layer of software testing. It involves testing individual units or components of the software in isolation to ensure they function correctly. In Rust, these tests are located in a separate test module within the same file as the code.

Tests can be run using the following command:
```bash
cargo test
```
Test failures are indicated by a `FAILED` message. The `--test-threads` flag can be used to specify the number of threads to use for running tests in parallel. The `--nocapture` flag can be used to show the output of a test that passes.

Sample test output (from the Rust doc):
```bash
running 2 tests
test tests::test_bad_add ... FAILED
test tests::test_add ... ok

failures:

---- tests::test_bad_add stdout ----
        thread 'tests::test_bad_add' panicked at 'assertion failed: `(left == right)`
  left: `-1`,
 right: `3`', src/lib.rs:21:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    tests::test_bad_add

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

It's not always possible and worth testing every single function in the code, especially when it comes to private functions. Our goal is to cover as many added or modified functions as it's possible, with 100% coverage on functions exported from each module.

## Integration tests
Integration testing is the next layer and focuses on verifying the interactions between different units or modules of the software. It ensures that integrated components work together as expected.

In Rust, these tests are written in separate files within the tests directory in the project root. Integration tests run in a separate process and are meant to test the public API of your code, simulating how the code will be used by other parts of the application or external clients.

Just like the unit tests, to run integration tests use the following command:
```bash
cargo test
```

The output will be similar to the unit tests ones, so each failing test will be indicated by a `FAILED` message.

The project aims to provide integration tests for each function exported from the crate or accessible by the external user. Tests for each function should cover at least one test case for each possible code path, including corner cases and at least positive and negative scenarios. At the end of the project, all tests should `PASS`.

Rust also supports "doctests," which are tests embedded in the code comments. Doctests are used to ensure that the code examples provided in the documentation remain accurate and up-to-date. We assume that each public function that is exported from the crate should include doctests.

To run doctests, use the following command:
```bash
cargo test --doc
```

## System (black-box) tests
System testing is conducted on the entire software system as a whole. It evaluates the behaviour and performance of the complete application against specified requirements.

Modified [node template](https://github.com/eigerco/substrate-node-template-move-vm-test) is meant to be used as a system testing framework. It's a fork of the plain Substrate template node with Move VM pallet handled in the runtime and node code. That repo gives the ability to test the Move VM pallet in the real environment, with the real Substrate node and real blockchain.

The node can be compiled and run following instructions from the [README](../README.md). It's also possible to run the node inside the Docker container.

Regardless of the method used to run the node, it's possible to connect to the node using the `Polkadot JS UI` or `Substrate Front End Template` and perform all extrinsic calls on the pallet. It means that there is a possibility to publish a new Move module and execute it later using the Move script. Sample scripts can be found on the Move website as well as in the testing assets directory in this repository.

Currently, user can call three extrinsics:
- `execute` - which executes the Move script;
- `publishModule` - which publishes a new Move module;
- `publishPackage` - which publishes a new Move package.

In Milestone 1, all implemented extrinsics return some dummy values, but are executed properly and generate events. The events are logged and can be seen in the UI.

User can fetch RPC methods using UI (Pallet Interactor -> Interaction type: RPC -> Pallet: rpc -> Callables: methods). It's possible to call the following methods:
- `mvm_estimateGasExecute`,
- `mvm_estimateGasPublish`,
- `mvm_gasToWeight`,
- `mvm_getModule`,
- `mvm_getModuleABI`,
- `mvm_getResource`,
- `mvm_weightToGas`.

Each RPC method can be called with the parameters. The parameters are passed as a JSON object. For example, to call `mvm_gasToWeight` with the `123` parameter use the following command:
```bash
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "mvm_gasToWeight", "params": [123]}' http://localhost:9944/
```

## Acceptance tests
Acceptance testing is the final testing layer before the software is released to end-users or stakeholders. It validates whether the software meets the intended business requirements and whether it is ready for production deployment.

The concrete set of acceptance tests will be defined in the future when the project will be closer to the production release. It's expected to define a full set of acceptance tests and agree on them with the client before the end of Milestone 2. Performing those tests will be a part of Milestone 3.

## Test automation
Test automation addresses some of the key challenges in traditional manual testing, such as time-consuming test execution, limited test coverage, and removes the potential human error factor. By automating repetitive and time-intensive test scenarios, teams can focus on more complex and exploratory testing, leading to a more thorough evaluation of the software's behaviour.

Not all tests are suitable for automation. We propose to choose tests that are stable, repeatable and have a high probability of catching defects. Complex and exploratory tests are better left to manual testing. In fact, unit, integration and many system tests can be automated, while acceptance tests are usually manual.

Unit and integration tests can be easily automated even using GitHub Actions and put directly into the workflow, even during normal build. It means that every time something new is done and pushed to the repository, it is possible to run tests on this latest build. Of course, it will use some resources (action minutes) to perform compilation and the whole test suite, so it should be considered only when merging to a specific branch or when doing a release.
