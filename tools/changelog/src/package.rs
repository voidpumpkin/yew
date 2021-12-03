use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Package {
    Yew,
    YewAgent,
    YewRouter,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Package::Yew => write!(f, "yew"),
            Package::YewAgent => write!(f, "yew-agent"),
            Package::YewRouter => write!(f, "yew-router"),
        }
    }
}

impl Package {
    pub fn as_labels(&self) -> Vec<String> {
        match self {
            Package::Yew => vec![
                "A-yew".to_string(),
                "A-yew-macro".to_string(),
                "macro".to_string(),
            ],
            Package::YewAgent => vec!["A-yew-agent".to_string()],
            Package::YewRouter => {
                vec!["A-yew-router".to_string(), "A-yew-router-macro".to_string()]
            }
        }
    }
}

impl TryFrom<&str> for Package {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "yew" => Ok(Package::Yew),
            "yew-agent" => Ok(Package::YewAgent),
            "yew-router" => Ok(Package::YewRouter),
            _ => Err(anyhow!("{} package is not supported for this cli", value)),
        }
    }
}
