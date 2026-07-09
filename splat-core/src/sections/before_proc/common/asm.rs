use std::sync::Arc;

use address_space::{AddressRange, Rom, RomVramRange, Size, Vram};
use anyhow::{Context, Result};
use spimdisasm::{
    rabbitizer::{InstructionFlags, IsaVersion},
    sections::before_proc::{ExecutableSection, ExecutableSectionSettings},
    segments::{OverlayCategoryName, ParentSegmentInfo},
};

use splat_segment_api::{section_trait::SectionTrait, segment_trait::SegmentGroup};

use crate::{config::instance::SplatInstance, sections::processed::common::CommonSegAsmProcessed};

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct CommonSegAsm {
    name: Arc<str>,
    section_type: Arc<str>,
    address: RomVramRange,

    spimdisasm_section: ExecutableSection,
}

impl SectionTrait for CommonSegAsm {
    fn name(&self) -> Arc<str> {
        Arc::clone(&self.name)
    }

    fn section_type(&self) -> Arc<str> {
        Arc::clone(&self.section_type)
    }

    fn rom(&self) -> Option<AddressRange<Rom>> {
        Some(*self.address.rom())
    }

    fn vram(&self) -> Option<AddressRange<Vram>> {
        Some(*self.address.vram())
    }
}

impl CommonSegAsm {
    pub fn new(
        splat_instance: &mut SplatInstance,
        name: impl Into<Arc<str>>,
        section_type: impl Into<Arc<str>>,
        raw_bytes: &[u8],
        rom: u32,
        vram_start: u32,
        most_parent: &impl SegmentGroup,
        // TODO: figure out these two
        args: &(),
        yaml: &(),
    ) -> Result<Self> {
        Self::new_impl(
            splat_instance,
            name.into(),
            section_type.into(),
            raw_bytes,
            rom,
            vram_start,
            most_parent,
            args,
            yaml,
        )
    }

    fn new_impl(
        splat_instance: &mut SplatInstance,
        name: Arc<str>,
        section_type: Arc<str>,
        raw_bytes: &[u8],
        rom: u32,
        vram_start: u32,
        most_parent: &impl SegmentGroup,
        // TODO: figure out these two
        _args: &(),
        _yaml: &(),
    ) -> Result<Self> {
        // TODO: tweak settings
        let section_settings =
            ExecutableSectionSettings::new(None, InstructionFlags::new(IsaVersion::MIPS_III));
        let parent_segment_info = ParentSegmentInfo::new(
            most_parent.rom().context("Missing Rom")?.start(),
            most_parent.vram_start().context("Missing Vram")?,
            most_parent
                .overlay_category_name()
                .map(OverlayCategoryName::new),
        );

        let address = RomVramRange::new_size(
            Rom::new(rom),
            Vram::new(vram_start),
            Size::new(raw_bytes.len() as u32),
            4, // mips asm is 4 aligned
        )
        .context("Invalid address")?;

        let spimdisasm_section = splat_instance.spimdisasm_context.create_section_text(
            &section_settings,
            Arc::clone(&name),
            raw_bytes.into(),
            address.rom().start(),
            address.vram().start(),
            parent_segment_info,
        )?;

        Ok(Self {
            name,
            section_type,
            address,

            spimdisasm_section,
        })
    }

    pub fn post_process(self, splat_instance: &mut SplatInstance) -> Result<CommonSegAsmProcessed> {
        let Self {
            name,
            section_type,
            address,
            spimdisasm_section,
        } = self;

        CommonSegAsmProcessed::new(
            splat_instance,
            name,
            section_type,
            address,
            spimdisasm_section,
        )
    }
}
