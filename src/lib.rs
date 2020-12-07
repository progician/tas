#![crate_name = "tas"]
#![crate_type = "lib"]
mod core;
mod matchers;

pub use core::*;
pub use matchers::equal_to::equal_to;


#[macro_export]
macro_rules! tests {
    ($body:block) => {
        fn main() {
            let mut test_cases: Vec<TestCase> = Vec::new();

            macro_rules! test_case {
                ($name:expr, $case_body:block) => {
                    test_cases.push(
                        TestCase {
                            name: $name.to_string(),
                            file: file!().to_string(),
                            line: line!(),
                            test_case_fn: || {
                                let mut results: AssertionResults = Vec::new();

                                macro_rules! require {
                                    ($expected:expr) => {
                                        let assertion_state =
                                            if $expected { AssertionState::Passed }
                                            else { AssertionState::Failed }
                                        ;
                                        results.push(AssertionResult {
                                            state: assertion_state,
                                            file: file!().to_string(),
                                            line: line!(),
                                            message: format!("`{}` is `{}`", stringify!($expected), $expected).to_string(),
                                            assertion_text: format!("require!({})", stringify!($expected)).to_string(),
                                        });
                                        if assertion_state == AssertionState::Failed {
                                            return results;
                                        }
                                    };
                                }

                                macro_rules! check {
                                    ($expected:expr) => {
                                        results.push(AssertionResult {
                                            state: if $expected { AssertionState::Passed } else { AssertionState::Failed },
                                            file: file!().to_string(),
                                            line: line!(),
                                            message: format!("`{}` is `false`", stringify!($expected).to_string()),
                                            assertion_text: format!("require!({})", stringify!($expected)).to_string(),
                                        });
                                    };
                                }

                                macro_rules! require_that {
                                    ($actual:expr, $matcher:expr) => ({
                                        let m = $matcher;
                                        match m.matches($actual) {
                                            Ok(_) => {
                                                results.push(AssertionResult {
                                                    state: AssertionState::Passed,
                                                    file: file!().to_string(),
                                                    line: line!(),
                                                    message: format!("condition was satisifed"),
                                                    assertion_text: format!("require_that!({}, {})", stringify!($actual), stringify!($matcher)).to_string(),
                                                });
                                            },
                                            Err(mismatch) => {
                                                results.push(AssertionResult {
                                                    state: AssertionState::Failed,
                                                    file: file!().to_string(),
                                                    line: line!(),
                                                    message: format!("Expected `{}` but {}", m, mismatch),
                                                    assertion_text: format!("require_that!({}, {})", stringify!($actual), stringify!($matcher)).to_string(),
                                                });
                                                return results;
                                            }
                                        }
                                    })
                                }

                                macro_rules! check_that {
                                    ($actual:expr, $matcher:expr) => ({
                                        let m = $matcher;
                                        match m.matches($actual) {
                                            Ok(_) => {
                                                results.push(AssertionResult {
                                                    state: AssertionState::Passed,
                                                    file: file!().to_string(),
                                                    line: line!(),
                                                    message: format!("condition was satisifed"),
                                                    assertion_text: format!("require_that!({}, {})", stringify!($actual), stringify!($matcher)).to_string(),
                                                });
                                            },
                                            Err(mismatch) => {
                                                results.push(AssertionResult {
                                                    state: AssertionState::Failed,
                                                    file: file!().to_string(),
                                                    line: line!(),
                                                    message: format!("Expected `{}` but {}", m, mismatch),
                                                    assertion_text: format!("require_that!({}, {})", stringify!($actual), stringify!($matcher)).to_string(),
                                                });
                                            }
                                        }
                                    })
                                }

                                $case_body;
                                results
                            },
                        }
                    );
                }
            }

            $body

            let mut overall_results = OverAllResults {
                test_cases: test_cases.len(),
                assertions: 0,
                failed_assertions: 0,
                failed: 0,
            };
            let test_case_matcher = |r: &AssertionResult| r.state == AssertionState::Failed;

            for case in test_cases {
                let test_case_assertions = (case.test_case_fn)();
                overall_results.assertions += test_case_assertions.len();
                let failed_assertions = test_case_assertions
                    .iter()
                    .filter(|r| r.state != AssertionState::Passed)
                    .count()
                ;

                overall_results.failed_assertions += failed_assertions;
                if failed_assertions != 0 {
                    overall_results.failed += 1;
                }

                let filtered_assertions: AssertionResults = test_case_assertions
                    .into_iter()
                    .filter(test_case_matcher)
                    .collect()
                ;
                if !filtered_assertions.is_empty() {
                    println!("{}", case);
                    for assertion_result in filtered_assertions {
                        println!("{}", assertion_result);
                    }
                }
            }

            println!("===============================================================================");
            println!("{}", overall_results);
        
            if !overall_results.failed != 0 {
                std::process::exit(1);
            }
            else {
                std::process::exit(0);
            }
        
        }
    }
}