use std::sync::Arc;

use address_space::{AddressRange, Rom, Size, Vram};

pub trait SegmentTrait {
    #[must_use]
    fn name(&self) -> Arc<str>;

    // #[must_use]
    // fn seg_type(&self) -> Arc<str>;

    #[must_use]
    fn rom(&self) -> Option<AddressRange<Rom>>;

    #[must_use]
    fn vram_start(&self) -> Option<Vram>;

    #[must_use]
    fn bss_size(&self) -> Option<Size>;

    #[must_use]
    fn rom_size(&self) -> Option<Size> {
        self.rom().map(|rom| rom.size())
    }

    #[must_use]
    fn size(&self) -> Option<Size> {
        match (self.rom_size(), self.bss_size()) {
            (None, None) => None,
            (None, Some(bss_size)) => Some(bss_size),
            (Some(rom_size), None) => Some(rom_size),
            (Some(rom_size), Some(bss_size)) => Some(rom_size.add_size(&bss_size)),
        }
    }

    #[must_use]
    fn vram_end(&self) -> Option<Vram> {
        let vram_start = self.vram_start()?;
        let size = self.size()?;

        Some(vram_start.add_size(&size))
    }
}

pub trait SegmentGroup: SegmentTrait {
    #[must_use]
    fn overlay_category_name(&self) -> Option<Arc<str>>;
}
