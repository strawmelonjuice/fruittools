use std::fs;

pub enum ToolChainName {
    // #1 Tool priority is gleam, second is rust, third is javascript etc. Yes. Opinion. Sorry! Not gonna change it.
    Gleam(ToolNameGleam),
    Rust(ToolNameRust),
}

impl ToolChainName {
    pub fn detect(dir: std::path::PathBuf) -> ToolChainName {
        if 
        std::fs::exists(dir.join("./gleam.toml"))
        {
            let gleam_manifest = dir.join("./gleam.toml");
            let gleam_manifest_string = fs::read_to_string(gleam_manifest.clone());
            
    }
}

pub(crate) enum ToolNameGleam {
    None,
    LustreDev,
    GleEscript,
}
pub(crate) enum ToolNameRust {
    None,
}
