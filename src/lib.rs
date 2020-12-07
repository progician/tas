// #![feature(trace_macros)]
//trace_macros!(true);

pub use std::collections::HashMap;
use std::fmt;


pub enum AssertionState {
    Passed,
    Failed,
}

pub struct AssertionResult {
    pub state: AssertionState,
    pub file: String,
    pub line: u32,
    pub message: String,
    pub assertion_text: String,
}


pub type AssertionResults = Vec<AssertionResult>;
pub type TestCaseBody = fn() -> AssertionResults;
pub struct TestCase {
    pub file: String,
    pub line: u32,
    pub test_case_fn: TestCaseBody,
}

pub type MatchResult = Result<(), String>;
pub trait Matcher<T> : fmt::Display {
    fn matches(&self, actual: T) -> MatchResult;
}

pub fn success() -> MatchResult {
    Ok(())
}

pub struct EqualTo<T> {
    expected: T,
}

impl<T: fmt::Debug> fmt::Display for EqualTo<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.expected.fmt(f)
    }
}

impl<T: PartialEq + fmt::Debug> Matcher<T> for EqualTo<T> {
    fn matches(&self, actual: T) -> MatchResult {
        if self.expected.eq(&actual) {
            success()
        } else {
            Err(format!("was {:?}", actual))
        }
    }
}


pub fn equal_to<T: PartialEq + fmt::Debug>(expected: T) -> EqualTo<T> {
    EqualTo { expected: expected }
}


#[macro_export]
macro_rules! tests {
    ($body:block) => {
        fn main() {
            let mut test_cases: HashMap<String, TestCase> = HashMap::new();

            macro_rules! test_case {
                ($name:expr, $case_body:block) => {
                    test_cases.insert($name.to_string(),
                        TestCase {
                            file: file!().to_string(),
                            line: line!(),
                            test_case_fn: || {
                                let mut results: AssertionResults = Vec::new();

                                macro_rules! require {
                                    ($expected:expr) => {
                                        if !$expected {
                                            results.push(AssertionResult {
                                                state: AssertionState::Failed,
                                                file: file!().to_string(),
                                                line: line!(),
                                                message: format!("`{}` is `false`", stringify!($expected).to_string()),
                                                assertion_text: format!("require!({})", stringify!($expected)).to_string(),
                                            });
                                            return results;
                                        }
                                    };
                                }

                                macro_rules! require_that {
                                    ($actual:expr, $matcher:expr) => ({
                                        let m = $matcher;
                                        match m.matches($actual) {
                                            Ok(_) => {},
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

            let mut all_succeeded = true;
            for (name, case) in test_cases {
                let test_case_assertions = (case.test_case_fn)();
                if !test_case_assertions.is_empty() {
                    println!("-------------------------------------------------------------------------------");
                    println!("{}", name);
                    println!("-------------------------------------------------------------------------------");
                    println!("{}:{}", case.file, case.line);
                    println!("...............................................................................");

                    for assertion_result in test_case_assertions {
                        println!("{}:{}", assertion_result.file, assertion_result.line);
                        println!("FAILED:");
                        println!("   {}", assertion_result.assertion_text);
                        println!("   {}", assertion_result.message);
                    }
        
                    all_succeeded = false;
                }
            }
        
            if !all_succeeded {
                std::process::exit(1);
            }
            else {
                std::process::exit(0);
            }
        
        }
    }
}