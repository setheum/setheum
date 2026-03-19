//! Integration tests for the move-bytecode module.
//!
//! Note:
//! These test heavily depend on Move projects within tests/assets/move-projects.
//! Some of these tests use addresses that need to match the address in Move project files -
//! otherwise executing scripts or publishing won't work as expected.

use move_vm_backend_common::bytecode::verify_script_integrity_and_check_signers;

/// Reads bytes from a file for the given path.
/// Panic if the file doesn't exist.
fn read_bytes(file_path: &str) -> Vec<u8> {
    std::fs::read(file_path)
        .unwrap_or_else(|e| panic!("Can't read {file_path}: {e} - make sure you run move-vm-backend/tests/assets/move-projects/smove-build-all.sh"))
}

/// Reads a precompiled Move scripts from our assets directory.
fn read_script_bytes_from_project(project: &str, script_name: &str) -> Vec<u8> {
    const MOVE_PROJECTS: &str = "tests/assets/move-projects";

    let path =
        format!("{MOVE_PROJECTS}/{project}/build/{project}/bytecode_scripts/{script_name}.mv");

    read_bytes(&path)
}

// General tests.

#[test]
fn general_invalid_script_fails() {
    // An invalid script like: "Bad cafe, aaaaa"!
    let script = vec![0xBA, 0xD, 0xCA, 0xFE, 0xAA, 0xAA, 0xAA, 0xAA];
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn general_script_no_params_works() {
    let script = read_script_bytes_from_project("signer_scripts", "no_param_at_all");
    let expect_no_signers = verify_script_integrity_and_check_signers(&script).unwrap();

    assert_eq!(expect_no_signers, 0);
}

#[test]
fn general_script_no_signers_param_at_all_works() {
    let script = read_script_bytes_from_project("signer_scripts", "no_signers_param_at_all");
    let expect_no_signers = verify_script_integrity_and_check_signers(&script).unwrap();

    assert_eq!(expect_no_signers, 0);
}

#[test]
fn general_script_eight_normal_signers_works() {
    let script = read_script_bytes_from_project("signer_scripts", "eight_normal_signers");
    let expect_eight_signers = verify_script_integrity_and_check_signers(&script).unwrap();

    assert_eq!(expect_eight_signers, 8);
}

#[test]
fn general_script_extra_signer_in_the_middle_of_the_list_fails() {
    let script =
        read_script_bytes_from_project("signer_scripts", "extra_signer_in_the_middle_of_the_list");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn general_script_trying_with_any_reference_fails() {
    let script = read_script_bytes_from_project("signer_scripts", "trying_with_integer_reference");
    let result = verify_script_integrity_and_check_signers(&script);
    assert!(result.is_err());

    let script = read_script_bytes_from_project("signer_scripts", "trying_with_addr_reference");
    let result = verify_script_integrity_and_check_signers(&script);
    assert!(result.is_err());

    let script =
        read_script_bytes_from_project("signer_scripts", "trying_with_mut_integer_reference");
    let result = verify_script_integrity_and_check_signers(&script);
    assert!(result.is_err());

    let script = read_script_bytes_from_project("signer_scripts", "trying_with_mut_addr_reference");
    let result = verify_script_integrity_and_check_signers(&script);
    assert!(result.is_err());
}

// An exclusive addition to the above test.
#[test]
fn general_script_trying_with_signer_reference_works() {
    let script = read_script_bytes_from_project("signer_scripts", "trying_with_signer_reference");
    let expect_one_signer = verify_script_integrity_and_check_signers(&script).unwrap();

    assert_eq!(expect_one_signer, 1);
}

#[test]
fn general_script_trying_with_mut_signer_reference_fails() {
    let script =
        read_script_bytes_from_project("signer_scripts", "trying_with_mut_signer_reference");
    let result = verify_script_integrity_and_check_signers(&script);
    assert!(result.is_err());
}

#[test]
fn general_script_trying_with_simple_struct_fails() {
    let script = read_script_bytes_from_project("signer_scripts", "trying_with_simple_struct");

    let result = verify_script_integrity_and_check_signers(&script);
    assert!(result.is_err());
}

#[test]
fn general_script_trying_with_struct_with_struct_members_fails() {
    let script =
        read_script_bytes_from_project("signer_scripts", "trying_with_struct_with_struct_members");

    let result = verify_script_integrity_and_check_signers(&script);
    assert!(result.is_err());
}

#[test]
fn general_script_trying_with_struct_with_generics_fails() {
    let script =
        read_script_bytes_from_project("signer_scripts", "trying_with_struct_with_generics");

    let result = verify_script_integrity_and_check_signers(&script);
    assert!(result.is_err());
}

// Generic tests.

#[test]
fn generic_script_signer_before_generic_fails() {
    let script = read_script_bytes_from_project("signer_scripts", "signer_before_generic");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn generic_script_two_signer_before_generic_fails() {
    let script = read_script_bytes_from_project("signer_scripts", "two_signers_before_generic");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn generic_script_signer_after_generic_fails() {
    let script = read_script_bytes_from_project("signer_scripts", "signer_after_generic");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn generic_script_signer_before_ref_generic_fails() {
    let script = read_script_bytes_from_project("signer_scripts", "signer_before_ref_generic");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn generic_script_simple_function_with_generic_inside_with_signer_param_works() {
    let script = read_script_bytes_from_project(
        "signer_scripts",
        "simple_function_with_generic_inside_with_signer_param",
    );
    let expect_one_signer = verify_script_integrity_and_check_signers(&script).unwrap();

    assert_eq!(expect_one_signer, 1);
}

#[test]
fn generic_script_simple_function_with_generic_inside_without_signer_param_works() {
    let script = read_script_bytes_from_project(
        "signer_scripts",
        "simple_function_with_generic_inside_without_signer_param",
    );
    let expect_no_signers = verify_script_integrity_and_check_signers(&script).unwrap();

    assert_eq!(expect_no_signers, 0);
}

// Vector tests.

#[test]
fn vector_script_signer_before_integer_vector_works() {
    let script = read_script_bytes_from_project("signer_scripts", "signer_before_integer_vector");
    let expect_one_signer = verify_script_integrity_and_check_signers(&script).unwrap();

    assert_eq!(expect_one_signer, 1);
}

#[test]
fn vector_script_signer_after_integer_vector_fails() {
    let script = read_script_bytes_from_project("signer_scripts", "signer_after_integer_vector");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn vector_script_signer_before_all_possible_vectors_works() {
    let script =
        read_script_bytes_from_project("signer_scripts", "signer_before_all_possible_vectors");
    let expect_one_signer = verify_script_integrity_and_check_signers(&script).unwrap();

    assert_eq!(expect_one_signer, 1);
}

#[test]
fn vector_script_signer_after_all_possible_vectors_fails() {
    let script =
        read_script_bytes_from_project("signer_scripts", "signer_after_all_possible_vectors");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn vector_script_signer_before_ref_vector_fails() {
    let script = read_script_bytes_from_project("signer_scripts", "signer_before_ref_vector");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn vector_script_signer_before_mut_ref_vector_fails() {
    let script = read_script_bytes_from_project("signer_scripts", "signer_before_mut_ref_vector");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn vector_script_trying_vector_containing_signer_fails() {
    let script =
        read_script_bytes_from_project("signer_scripts", "trying_vector_containing_signer");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn vector_script_trying_vector_containing_struct_fails() {
    let script =
        read_script_bytes_from_project("signer_scripts", "trying_vector_containing_struct");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn vector_script_trying_vector_containing_struct_with_struct_fails() {
    let script = read_script_bytes_from_project(
        "signer_scripts",
        "trying_vector_containing_struct_with_struct",
    );
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn vector_script_trying_vector_containing_struct_with_generic_fails() {
    let script = read_script_bytes_from_project(
        "signer_scripts",
        "trying_vector_containing_struct_with_generic",
    );
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}

#[test]
fn vector_script_trying_vector_containing_address_vector_works() {
    let script =
        read_script_bytes_from_project("signer_scripts", "trying_vector_containing_address_vector");
    let expect_no_signers = verify_script_integrity_and_check_signers(&script).unwrap();

    assert_eq!(expect_no_signers, 0);
}

#[test]
fn vector_script_trying_vector_containing_vector_containing_u8_vector_works() {
    let script = read_script_bytes_from_project(
        "signer_scripts",
        "trying_vector_containing_vector_containing_u8_vector",
    );
    let expect_no_signers = verify_script_integrity_and_check_signers(&script).unwrap();

    assert_eq!(expect_no_signers, 0);
}

#[test]
fn vector_script_trying_vector_containing_signer_vector_fails() {
    let script =
        read_script_bytes_from_project("signer_scripts", "trying_vector_containing_signer_vector");
    let result = verify_script_integrity_and_check_signers(&script);

    assert!(result.is_err());
}
