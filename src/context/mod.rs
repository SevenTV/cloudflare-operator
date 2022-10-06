use crate::config::Config;
use crate::types::Result;

mod run;

pub struct Context {
    pub config: Config,
}

impl Context {
    pub async fn run(&self) -> Result<()> {
        run::run(self).await
    }
}
