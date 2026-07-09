use std::{
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
    sync::Arc,
};

use address_space::{AddressRange, Rom, RomVramRange, Vram};
use anyhow::{Context, Result};
use spimdisasm::{
    sections::{before_proc::DataSection, processed::DataSectionProcessed},
    symbols::display::SymDataDisplaySettings,
};

use splat_segment_api::section_trait::SectionTrait;

use crate::config::instance::SplatInstance;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct CommonSegDataProcessed {
    name: Arc<str>,
    section_type: Arc<str>,
    address: RomVramRange,
    path: PathBuf,

    spimdisasm_section: DataSectionProcessed,
}

impl SectionTrait for CommonSegDataProcessed {
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

impl CommonSegDataProcessed {
    pub(crate) fn new(
        splat_instance: &mut SplatInstance,
        name: Arc<str>,
        section_type: Arc<str>,
        address: RomVramRange,
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
            section_type,
            address,
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
            writeln!(writer, "{}", sym_display)?;
        }

        Ok(())
    }
}
