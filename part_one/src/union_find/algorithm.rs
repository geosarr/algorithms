use std::fmt;
use std::str::FromStr;
use std::error::Error;


#[derive(Copy, Clone, Debug)]
pub enum Algorithm {
    QuickFind, 
    QuickUnion, 
    WeightedQuickUnion, 
    WeightedQuickUnionPathComp
}
const ALGORITHMS: &str = "[QuickFind, QuickUnion, WeightedQuickUnion, WeightedQuickUnionPathComp]";

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Algorithm::QuickFind => write!(f, "QuickFind"),
            Algorithm::QuickUnion => write!(f, "QuickUnion"),
            Algorithm::WeightedQuickUnion => write!(f, "WeightedQuickUnion"),
            Algorithm::WeightedQuickUnionPathComp => write!(f, "WeightedQuickUnionPathComp"),
        }
    }
}

impl FromStr for Algorithm {
    type Err = ParseAlgorithmError;

    fn from_str(s: &str) -> std::result::Result<Self, ParseAlgorithmError> {
        match s {
            "QuickFind" => Ok(Algorithm::QuickFind),
            "QuickUnion" => Ok(Algorithm::QuickUnion),
            "WeightedQuickUnion" => Ok(Algorithm::WeightedQuickUnion),
            "WeightedQuickUnionPathComp" => Ok(Algorithm::WeightedQuickUnionPathComp),
            _ => Err(ParseAlgorithmError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseAlgorithmError;

impl fmt::Display for ParseAlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc_init0: &str = "algorithm spelling incorrect, only available are";
        let desc_init1 = format!("{} {}", desc_init0, ALGORITHMS);
        let description = desc_init1.as_str();
        f.write_str(description)
    }
}

impl Error for ParseAlgorithmError {}