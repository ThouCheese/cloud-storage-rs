use std::str::FromStr;

/// Any type of team we can encounter.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Team {
    /// The team consists of `Editors`.
    Editors,
    /// The team consists of `Owners`.
    Owners,
    /// The team consists of `Viewers`.
    Viewers,
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Team::Editors => write!(f, "editors"),
            Team::Owners => write!(f, "owners"),
            Team::Viewers => write!(f, "viewers"),
        }
    }
}

impl FromStr for Team {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "editors" => Ok(Self::Editors),
            "owners" => Ok(Self::Owners),
            "viewers" => Ok(Self::Viewers),
            _ => Err(format!("Invalid `Team`: {}", s)),
        }
    }
}