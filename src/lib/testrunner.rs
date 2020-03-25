use failure::Error;

use crate::lib::Config;

#[derive(Debug, Clone)]
pub(crate) struct MutateTester {
    config: Config,
}

impl MutateTester {
    pub(crate) fn new(config: Config) -> Self {
        Self { config }
    }

    // pub(crate) fn reduce(&self) -> Result<RustSource, Error> {
    //     // TODO: implement me
    // }
}
