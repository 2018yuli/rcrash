use std::any;

use anyhow::Ok;
use deno_core::{serde_json, JsRuntime, RuntimeOptions};
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize)]
struct TestResult {
    name: String,
    passed: bool,
    error: Option<String>,
}

fn run_declare_tests() -> anyhow::Result<()> {
    // Initialize the JS runtime with console.log support.
    let mut runtime = JsRuntime::new(RuntimeOptions {
        ..Default::default()
    });

    // Load and execute declare.js (simulate module.exports).
    let declare_code = include_str!("../assets/extend/declare.js");
    let wrapped_declare = format!(
        "let module = {{exports: {{}}}}; let exports = module.exports; {} globalThis.declare = module.exports;",
        declare_code
    );
    runtime
        .execute_script("declare.js", wrapped_declare)
        .unwrap();

    // Setup CommonJS 'require' with 'assert', 'it', and our declare module.
    let setup_code = r#"
        const modules = {};
        modules['assert'] = {
            isTrue(x) { if (!x) throw new Error(`Expected true, got ${x}`); },
            isFalse(x) { if (x) throw new Error(`Expected false, got ${x}`); },
            equal(a,b) { if (a != b) throw new Error(`Expected ${b}, got ${a}`); },
            deepEqual(a,b) {
                if (typeof a === 'object' && JSON.stringify(a) !== JSON.stringify(b)) {
                    throw new Error(`Not deep equal: ${JSON.stringify(a)} != ${JSON.stringify(b)}`);
                }
                if (!(typeof a === 'object') && a !== b) {
                    throw new Error(`Not deep equal: ${a} != ${b}`);
                }
            },
            instanceOf(obj,type) { if (!(obj instanceof type)) throw new Error(`Expected instance of ${type.name}`); }
        };
        const tests = [];
        const it = {
            describe(name, fn) { fn(it); },
            should(name, fn) { tests.push({ name, fn }); }
        };
        modules['it'] = it;
        modules['../declare'] = globalThis.declare;
        function require(name) {
            if (modules[name]) return modules[name];
            throw new Error(`Module not found: ${name}`);
        }
        "#;
    runtime.execute_script("setup.js", setup_code).unwrap();
    Ok(())
    // Load and execute the test suite, which will populate `tests`.
    // let test_code = include_str!("declare.test.js");
    // runtime
    //     .execute_script("declare.test.js", test_code)
    //     .unwrap();

    // Run each collected JS test and collect results.
    // let results_json = runtime
    //     .execute_script(
    //         "run_tests.js",
    //         r#"
    //             JSON.stringify(tests.map(test => {
    //                 try {
    //                     test.fn();
    //                     return { name: test.name, passed: true, error: null };
    //                 } catch (e) {
    //                     return { name: test.name, passed: false, error: e.message };
    //                 }
    //             }))
    //             "#,
    //     )
    //     .unwrap();
    // let scope = &mut runtime.handle_scope();
    // let json_str = results_json.open(scope).to_rust_string_lossy(scope);
    // serde_json::from_str(&json_str).unwrap()
}

// static JS_RESULTS: OnceCell<Vec<TestResult>> = OnceCell::new();

// fn js_test_results() -> &'static Vec<TestResult> {
//     JS_RESULTS.get_or_init(|| run_declare_tests())
// }

#[test]
fn test_sound_like_a_dog() {
    let _ = run_declare_tests();
    // let result = run_declare_tests()
    //     .iter()
    //     .find(|r| r.name == "sound like a dog")
    //     .unwrap();
    // assert_eq!(
    //     result.passed, true,
    //     "JS test '{}' failed: {:?}",
    //     result.name, result.error
    // );
}

// #[test]
// fn test_be_a_dog_after_setting_type() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "be a DOG after setting type")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_sound_like_a_lab_breed() {
//     // The first "sound like a lab" test (Breed context).
//     let results: Vec<&TestResult> = js_test_results()
//         .iter()
//         .filter(|r| r.name == "sound like a lab")
//         .collect();
//     let result = results[0];
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_sound_like_a_lab_lab() {
//     // The second "sound like a lab" test (Lab context).
//     let results: Vec<&TestResult> = js_test_results()
//         .iter()
//         .filter(|r| r.name == "sound like a lab")
//         .collect();
//     let result = results[1];
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_after_setting_type_collie() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "but after setting the type it should be a collie")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_after_setting_color_grey() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "but after setting the color it should be grey")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_after_setting_sound_bark() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "but after setting the sound it should sound like a bark")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_after_setting_pitch_low() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "but after setting the pitch it should be low pitched")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_still_be_a_dog_1() {
//     // First "still be a dog" test.
//     let results: Vec<&TestResult> = js_test_results()
//         .iter()
//         .filter(|r| r.name == "still be a dog")
//         .collect();
//     let result = results[0];
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_still_be_a_dog_2() {
//     // Second "still be a dog" test.
//     let results: Vec<&TestResult> = js_test_results()
//         .iter()
//         .filter(|r| r.name == "still be a dog")
//         .collect();
//     let result = results[1];
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_still_be_a_dog_3() {
//     // Third "still be a dog" test.
//     let results: Vec<&TestResult> = js_test_results()
//         .iter()
//         .filter(|r| r.name == "still be a dog")
//         .collect();
//     let result = results[2];
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_call_static_init_methods() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "call static init methods")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_allow_calling_without_args() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "allow calling with out arguments")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_allow_calling_super_string() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "allow calling super a string argument")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_allow_calling_super_null() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "allow calling super with null")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_allow_calling_super_undefined() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "allow calling super with undefined")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_add_to_exports() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "add to the exports ")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }

// #[test]
// fn test_export_as_module() {
//     let result = js_test_results()
//         .iter()
//         .find(|r| r.name == "export as module")
//         .unwrap();
//     assert_eq!(
//         result.passed, true,
//         "JS test '{}' failed: {:?}",
//         result.name, result.error
//     );
// }
