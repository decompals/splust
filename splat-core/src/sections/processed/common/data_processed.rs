use std::{
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
    sync::Arc,
};

use anyhow::{Context, Result};
use spimdisasm::{
    sections::{before_proc::DataSection, processed::DataSectionProcessed},
    symbols::display::SymDataDisplaySettings,
};

use splat_segment_api::segment_trait::SegmentTrait;

use crate::config::instance::SplatInstance;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct CommonSegDataProcessed {
    name: Arc<str>,
    seg_type: Arc<str>,
    rom: (u32, u32),
    vram_start: u32,
    path: PathBuf,

    spimdisasm_section: DataSectionProcessed,
}

impl SegmentTrait for CommonSegDataProcessed {
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

impl CommonSegDataProcessed {
    pub(crate) fn new(
        splat_instance: &mut SplatInstance,
        name: Arc<str>,
        seg_type: Arc<str>,
        rom: (u32, u32),
        vram_start: u32,
        spimdisasm_section: DataSection,
    ) -> Result<Self> {
        let spimdisasm_processed = spimdisasm_section.post_process(
            &mut splat_instance.spimdisasm_context,
            &splat_instance.user_relocs,
        )?;

        // TODO: self.dir in the middle
        // options.opts.asm_path / self.dir / f"{self.name}.s"
        let path = splat_instance.options.asm_path.join(format!("{}.s", name));

        Ok(Self {
            name,
            seg_type,
            rom,
            vram_start,
            path,

            spimdisasm_section: spimdisasm_processed,
        })
    }

    pub fn split(&self, splat_instance: &SplatInstance) -> Result<()> {
        fs::create_dir_all(self.path.parent().context("unable to get parent dir?")?)?;

        let mut writer = BufWriter::new(fs::File::create(&self.path)?);
        // TODO: get_asm_file_header

        let data_settings = SymDataDisplaySettings::new();

        for sym in self.spimdisasm_section.data_symbols() {
            let sym_display = sym.display(&splat_instance.spimdisasm_context, &data_settings)?;
            write!(writer, "{}\n", sym_display)?;
        }

        Ok(())
    }
}
