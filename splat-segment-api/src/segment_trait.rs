use std::sync::Arc;

pub trait SegmentTrait {
    #[must_use]
    fn name(&self) -> Arc<str>;

    #[must_use]
    fn seg_type(&self) -> Arc<str>;

    #[must_use]
    fn rom(&self) -> Option<(u32, u32)>;

    #[must_use]
    fn vram_start(&self) -> Option<u32>;

    #[must_use]
    fn bss_size(&self) -> Option<u32>;


    #[must_use]
    fn rom_size(&self) -> Option<u32> {
        self.rom().map(|(start, end)| end.wrapping_sub(start))
    }

    #[must_use]
    fn size(&self) -> Option<u32> {
        match (self.rom_size(), self.bss_size()) {
            (None, None) => None,
            (None, Some(bss_size)) => Some(bss_size),
            (Some(rom_size), None) => Some(rom_size),
            (Some(rom_size), Some(bss_size)) => Some(rom_size.wrapping_add(bss_size)),
        }
    }

    #[must_use]
    fn vram_end(&self) -> Option<u32> {
        let vram_start = self.vram_start()?;
        let size = self.size()?;

        Some(vram_start.wrapping_add(size))
    }
}

pub trait SegmentGroup: SegmentTrait {
    #[must_use]
    fn overlay_category_name(&self) -> Option<Arc<str>>;
}

