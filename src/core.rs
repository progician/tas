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
