use crate::fruittools_shared::FruitToolsUnifiedErrorType;
use crate::fruittools_shared::FruitToolsUnifiedErrorType::IOError;
use crate::walkdir::WalkDir;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolChainName {
    // #1 Tool priority is gleam, second is rust, third is javascript etc. Yes. Opinion. Sorry! Not gonna change it.
    Gleam(ToolNameGleam),
    Rust(ToolNameRust),
}

pub enum Target {
    Dev,
    Build,
    Watch,
}

impl ToolChainName {
    pub fn detect_all(
        dir: std::path::PathBuf,
    ) -> Result<Vec<ToolChainName>, FruitToolsUnifiedErrorType> {
        let mut found = vec![];
        if std::fs::exists(dir.join("./gleam.toml")).map_err(IOError)?
        // If the gleam.toml file exists, we can assume that the toolchain is gleam.
        {
            // Ding! One in the pocket already!
            found.push(ToolChainName::Gleam(ToolNameGleam::None));
            // A more advanced version of the gleam detection might be implemented in the future.
            // You know, one that also actually parses the gleam.toml file and checks for the presence of dependencies.
            let gleam_manifest = dir.join("./gleam.toml");
            let gleam_manifest_string =
                fs::read_to_string(gleam_manifest.clone()).map_err(|e| {
                    return FruitToolsUnifiedErrorType::UnreadableFile {
                        file: gleam_manifest.clone(),
                        io_error: e,
                        msg: "Gleam manifest unreadable.".to_string(),
                    };
                })?;
            if gleam_manifest_string.contains("gleescript =") {
                found.push(ToolChainName::Gleam(ToolNameGleam::GleEscript))
            }
            if gleam_manifest_string.contains("lustre_dev_tools =") {
                found.push(ToolChainName::Gleam(ToolNameGleam::LustreDev))
            }
        }
        if std::fs::exists(dir.join("./Cargo.toml")).map_err(IOError)? {
            // If the Cargo.toml file exists, we can assume that the toolchain is rust.
            found.push(ToolChainName::Rust(ToolNameRust::None));
            {
                // If recursively ANY of the .rs files in the src directory contains a test function, we can assume that the toolchain has rust tests.
                let src_dir = dir.join("./src");
                for f in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
                    if f.metadata().unwrap().is_file() {
                        if fs::read_to_string(f.path())
                            .map_err(|er| {
                                return FruitToolsUnifiedErrorType::UnreadableFile {
                                    file: f.path().to_path_buf(),
                                    io_error: er,
                                    msg: "Rust source file unreadable.".to_string(),
                                };
                            })?
                            .contains("#[test]")
                        {
                            found.push(ToolChainName::Rust(ToolNameRust::Tests));
                            break;
                        }
                    }
                }
            }
        }
        Ok(found)
    }
    pub fn detect(
        dir: std::path::PathBuf,
        target: Target,
    ) -> Result<(u16, ToolChainName), FruitToolsUnifiedErrorType> {
        let found = ToolChainName::detect_all(dir)?;

        {
            if found.len() == 0 {
                return Err(FruitToolsUnifiedErrorType::NoManifestRecognized);
            }
            let mut filter: Vec<(u16, ToolChainName)> = {
                match target {
                    Target::Dev => {
                        vec![
                            (6, ToolChainName::Gleam(ToolNameGleam::LustreDev)),
                            (3, ToolChainName::Gleam(ToolNameGleam::None)),
                            (4, ToolChainName::Rust(ToolNameRust::None)),
                            (5, ToolChainName::Rust(ToolNameRust::Tests)),
                        ]
                    }
                    Target::Build => {
                        vec![
                            (6, ToolChainName::Gleam(ToolNameGleam::GleEscript)),
                            (6, ToolChainName::Rust(ToolNameRust::None)),
                            (5, ToolChainName::Gleam(ToolNameGleam::LustreDev)),
                            (3, ToolChainName::Gleam(ToolNameGleam::None)),
                        ]
                    }
                    Target::Watch => {
                        todo!()
                    }
                }
            };
            filter.sort_by(|a, b| (b.0.cmp(&a.0)));
            if cfg!(test) {
                println!(
                    "Sorted filter: {:?}\nComparing against: {:?}",
                    filter, found
                );
            }
            // We need to find the first element in filter that is also in found.
            let res = filter.iter().find(|x| found.contains(&x.1));
            if cfg!(test) {
                println!("res: {:?}", res);
            }
            if res.is_none() {
                Err(FruitToolsUnifiedErrorType::NoManifestRecognized)
            } else {
                Ok(res.unwrap().clone())
            }
        }
    }
    pub fn display(&self) -> String {
        match self {
            ToolChainName::Gleam(tool) => match tool {
                ToolNameGleam::None => "Gleam".to_string(),
                ToolNameGleam::LustreDev => "Gleam with Lustre".to_string(),
                ToolNameGleam::GleEscript => "Gleam with gleescript".to_string(),
            },
            ToolChainName::Rust(tool) => match tool {
                ToolNameRust::None => "Rust".to_string(),
                ToolNameRust::Tests => "Rust with tests".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ToolNameGleam {
    None,
    LustreDev,
    GleEscript,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ToolNameRust {
    Tests,
    None,
}
