use anyhow::Result;

use std::fmt;

#[derive(Debug,Clone,Default,PartialEq)]
pub enum Stage{
    Local,
    #[default]
    Dev,
    Prod,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let stage = match self {
            Stage::Local => "Local",
            Stage::Dev => "Dev",
            Stage::Prod => "Prod" ,
        };
        write!(f,"{}",stage)
    }
}

impl Stage {
    pub fn try_form(stage: &str) -> Result<Self> {
        match stage {
            "Local" => Ok(Self::Local),
            "Dev" => Ok(Self::Dev),
            "Prod" => Ok(Self::Prod),
            _ => Err(anyhow::anyhow!("Invalid stage")),
        }
    }
}