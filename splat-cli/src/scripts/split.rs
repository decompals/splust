use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use splat_core::yaml;

#[derive(Debug, Clone, Args)]
pub struct SplitArgs {
    #[arg(required = true)]
    //pub config: Vec<PathBuf>,
    config: PathBuf,

    #[arg(long, default_values_t = ["all".to_string()])]
    modes: Vec<String>,

    // #[arg(long)]
    // verbose: bool,
    // #[arg(long)]
    // use_cache: bool,
    // #[arg(long)]
    // disassemble_all: bool,
    // #[arg(long)]
    // make_full_disasm_for_code: bool,
}

impl SplitArgs {
    // TODO: rename
    pub fn do_stuff(&self) -> Result<()> {
        let _splat_yaml = yaml::load_yaml(&self.config)?;

        Ok(())
    }
}
