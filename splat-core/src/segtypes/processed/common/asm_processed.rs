use std::sync::Arc;

use splat_segment_api::segment_trait::SegmentTrait;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommonSegAsmProcessed {
    name: Arc<str>,
    seg_type: Arc<str>,
    rom: (u32, u32),
    vram_start: u32,
}

impl SegmentTrait for CommonSegAsmProcessed {
    fn name(&self) -> Arc<str> {
        Arc::clone(&self.name)
    }

    fn seg_type(&self) -> Arc<str> {
        Arc::clone(&self.seg_type)
    }

    fn rom(&self) -> Option<(u32, u32)> {
        Some(self.rom)
    }

    fn vram_start(&self) -> Option<u32> {
        Some(self.vram_start)
    }

    fn bss_size(&self) -> Option<u32> {
        None
    }
}
