use std::sync::Arc;

use anyhow::{Context, Result};
use spimdisasm::{
    addresses::{Rom, Vram},
    metadata::OverlayCategoryName,
    parent_segment_info::ParentSegmentInfo,
    rabbitizer::{InstructionFlags, IsaVersion},
    sections::before_proc::{ExecutableSection, ExecutableSectionSettings},
};

use splat_segment_api::segment_trait::{SegmentGroup, SegmentTrait};

use crate::{config::instance::SplatInstance, sections::processed::common::CommonSegAsmProcessed};

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct CommonSegAsm {
    name: Arc<str>,
    seg_type: Arc<str>,
    rom: (u32, u32),
    vram_start: u32,

    spimdisasm_section: ExecutableSection,
}

impl SegmentTrait for CommonSegAsm {
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

impl CommonSegAsm {
    pub fn new(
        splat_instance: &mut SplatInstance,
        name: impl Into<Arc<str>>,
        seg_type: impl Into<Arc<str>>,
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
            seg_type.into(),
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
        seg_type: Arc<str>,
        raw_bytes: &[u8],
        rom: u32,
        vram_start: u32,
        most_parent: &impl SegmentGroup,
        // TODO: figure out these two
        _args: &(),
        _yaml: &(),
    ) -> Result<Self> {
        let text_settings =
            ExecutableSectionSettings::new(None, InstructionFlags::new(IsaVersion::MIPS_III));
        let parent_segment_info = ParentSegmentInfo::new(
            Rom::new(most_parent.rom().context("Missing Rom")?.0),
            Vram::new(most_parent.vram_start().context("Missing Vram")?),
            most_parent
                .overlay_category_name()
                .map(OverlayCategoryName::new),
        );

        let spimdisasm_section = splat_instance.spimdisasm_context.create_section_text(
            &text_settings,
            Arc::clone(&name),
            raw_bytes.into(),
            Rom::new(rom),
            Vram::new(vram_start),
            parent_segment_info,
        )?;

        Ok(Self {
            name,
            seg_type,
            rom: (rom, rom.wrapping_add(raw_bytes.len() as u32)),
            vram_start,

            spimdisasm_section,
        })
    }

    pub fn post_process(self, splat_instance: &mut SplatInstance) -> Result<CommonSegAsmProcessed> {
        let Self {
            name,
            seg_type,
            rom,
            vram_start,
            spimdisasm_section,
        } = self;

        CommonSegAsmProcessed::new(
            splat_instance,
            name,
            seg_type,
            rom,
            vram_start,
            spimdisasm_section,
        )
    }
}
