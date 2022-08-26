use std::fmt;

/// States of a service
#[derive(Clone)]
pub enum ServiceStatus {
    Critical,
    Passing,
    Warning,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        ServiceStatus::Critical
    }
}

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceStatus::Critical => write!(f, "critical"),
            ServiceStatus::Passing => write!(f, "passing"),
            ServiceStatus::Warning => write!(f, "warning"),
        }
    }
}

impl From<String> for ServiceStatus {
    fn from(v: String) -> Self {
        if v == "warning" {
            return ServiceStatus::Warning;
        } else if v == "passing" {
            return ServiceStatus::Passing;
        }

        ServiceStatus::Critical
    }
}
