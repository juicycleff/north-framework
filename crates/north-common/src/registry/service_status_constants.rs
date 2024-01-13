use std::fmt;

/// States of a service
#[derive(Clone)]
#[derive(Default)]
pub enum ServiceStatus {
    #[default]
    Critical,
    Passing,
    Warning,
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
