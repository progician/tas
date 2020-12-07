pub use std::collections::HashMap;
use std::fmt;


#[derive(PartialEq, Clone, Copy)]
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

impl fmt::Display for AssertionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
"
{}:{}
{}
    {}
with
    {}
",
            self.file, self.line,
            match self.state {
                AssertionState::Failed => "FAILED:",
                AssertionState::Passed => "PASSED:",
            },
            self.assertion_text,
            self.message
        )
    }
}


pub type AssertionResults = Vec<AssertionResult>;
pub type TestCaseBody = fn() -> AssertionResults;
pub struct TestCase {
    pub name: String,
    pub file: String,
    pub line: u32,
    pub test_case_fn: TestCaseBody,
}

impl fmt::Display for TestCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
"
-------------------------------------------------------------------------------
{}
-------------------------------------------------------------------------------
{}:{}
..............................................................................."
            , self.name, self.file, self.line
        )
    }
}

pub type MatchResult = Result<(), String>;
pub trait Matcher<T> : fmt::Display {
    fn matches(&self, actual: &T) -> MatchResult;
}

pub fn success() -> MatchResult {
    Ok(())
}


pub struct OverAllResults {
    pub test_cases: usize,
    pub assertions: usize,
    pub failed_assertions: usize,
    pub failed: usize,
}

impl fmt::Display for OverAllResults {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.test_cases == 0 {
            write!(f, "No tests were run")
        }
        else if self.failed == 0 {
            write!(f,
                "All tests passed ({} assertions in {} test cases)",
                self.assertions,
                self.test_cases
            )
        }
        else {
            write!(f,
                "{} tests failed out of {} test cases ({} failed in {} assertions)",
                self.failed,
                self.test_cases,
                self.failed_assertions,
                self.assertions
            )
        }
    }
}