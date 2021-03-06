use failure::Error;

use super::EngineOptions;
use core::SharedState;

pub trait EngineTrait {
    fn execute(
        &mut self,
        index: usize,
        state: SharedState,
        engine_options: EngineOptions,
    ) -> Result<(), Error>;
}
