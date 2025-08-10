use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Default for Status {
    fn default() -> Self {
        Self::Open
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            Self::Open => "OPEN".blue(),
            Self::InProgress => "IN PROGRESS".yellow(),
            Self::Resolved => "RESOLVED".green(),
            Self::Closed => "Closed".red(),
        };

        write!(f, "{}", status_str)
    }
}
