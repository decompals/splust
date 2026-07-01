use std::collections::BTreeMap;

use spimdisasm::{addresses::Rom, context::Context as SpimdisasmContext, relocation::RelocationInfo};

use super::options::SplatOpts;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct SplatInstance {
    pub(crate) options: SplatOpts,
    pub(crate) yaml_segments: (),
    pub(crate) symbols: (),
    pub(crate) spimdisasm_context: SpimdisasmContext,
    pub(crate) user_relocs: BTreeMap<Rom, RelocationInfo>, // TODO: Change this into a proper type in spimdisasm
}

impl SplatInstance {
    pub fn options(&self) -> &SplatOpts {
        &self.options
    }
    pub fn spimdisasm_context(&self) -> &SpimdisasmContext {
        &self.spimdisasm_context
    }
    pub fn user_relocs(&self) -> &BTreeMap<Rom, RelocationInfo> {
        &self.user_relocs
    }
}
