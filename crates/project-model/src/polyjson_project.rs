use base_db::{CrateDisplayName, CrateId, CrateName, Dependency, Edition};
use la_arena::RawIdx;
use paths::{AbsPath, AbsPathBuf};
use rustc_hash::FxHashMap;
use serde::{de, Deserialize};
use std::path::PathBuf;

use crate::cfg_flag::CfgFlag;

/// Roots and crates that compose this Rust project.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolyJsonProject {
    /// e.g. `path/to/sysroot`
    pub(crate) sysroot: Option<AbsPathBuf>,
    /// e.g. `path/to/sysroot/lib/rustlib/src/rust`
    pub(crate) sysroot_src: Option<AbsPathBuf>,
    project_root: AbsPathBuf,
    crates: Vec<PolyJsonCrate>,
}

/// A crate points to the root module of a crate and lists the dependencies of the crate. This is
/// useful in creating the crate graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolyJsonCrate {
    pub(crate) display_name: Option<CrateDisplayName>,
    pub(crate) root_module: AbsPathBuf,
    pub(crate) edition: Edition,
    pub(crate) version: Option<String>,
    pub(crate) deps: Vec<Dependency>,
    pub(crate) cfg: Vec<CfgFlag>,
    pub(crate) target: Option<String>,
    pub(crate) env: FxHashMap<String, String>,
    pub(crate) proc_macro_dylib_path: Option<AbsPathBuf>,
    pub(crate) is_workspace_member: bool,
    pub(crate) include: Vec<AbsPathBuf>,
    pub(crate) exclude: Vec<AbsPathBuf>,
    pub(crate) is_proc_macro: bool,
    pub(crate) repository: Option<String>,
}

impl PolyJsonProject {
    /// Create a new PolyJsonProject instance.
    ///
    /// # Arguments
    ///
    /// * `base` - The path to the workspace root (i.e. the folder containing `rust-project.json`)
    /// * `data` - The parsed contents of `rust-project.json`, or project json that's passed via
    ///            configuration.
    pub fn new(base: &AbsPath, data: PolyJsonData) -> PolyJsonProject {
        let absolutize_on_base = |p| base.absolutize(p);
        PolyJsonProject {
            sysroot: data.sysroot.map(absolutize_on_base),
            sysroot_src: data.sysroot_src.map(absolutize_on_base),
            project_root: base.to_path_buf(),
            crates: data
                .crates
                .into_iter()
                .map(|crate_data| {
                    let root_module = absolutize_on_base(crate_data.root_module);
                    let is_workspace_member = crate_data
                        .is_workspace_member
                        .unwrap_or_else(|| root_module.starts_with(base));
                    let (include, exclude) = match crate_data.source {
                        Some(src) => {
                            let absolutize = |dirs: Vec<PathBuf>| {
                                dirs.into_iter().map(absolutize_on_base).collect::<Vec<_>>()
                            };
                            (absolutize(src.include_dirs), absolutize(src.exclude_dirs))
                        }
                        None => (vec![root_module.parent().unwrap().to_path_buf()], Vec::new()),
                    };

                    PolyJsonCrate {
                        display_name: crate_data
                            .display_name
                            .map(CrateDisplayName::from_canonical_name),
                        root_module,
                        edition: crate_data.edition.into(),
                        version: crate_data.version.as_ref().map(ToString::to_string),
                        deps: crate_data
                            .deps
                            .into_iter()
                            .map(|dep_data| {
                                Dependency::new(
                                    dep_data.name,
                                    CrateId::from_raw(RawIdx::from(dep_data.krate as u32)),
                                )
                            })
                            .collect::<Vec<_>>(),
                        cfg: crate_data.cfg,
                        target: crate_data.target,
                        env: crate_data.env,
                        proc_macro_dylib_path: crate_data
                            .proc_macro_dylib_path
                            .map(absolutize_on_base),
                        is_workspace_member,
                        include,
                        exclude,
                        is_proc_macro: crate_data.is_proc_macro,
                        repository: crate_data.repository,
                    }
                })
                .collect(),
        }
    }

    /// Returns the number of crates in the project.
    pub fn n_crates(&self) -> usize {
        self.crates.len()
    }

    /// Returns an iterator over the crates in the project.
    pub fn crates(&self) -> impl Iterator<Item = (CrateId, &PolyJsonCrate)> + '_ {
        self.crates
            .iter()
            .enumerate()
            .map(|(idx, krate)| (CrateId::from_raw(RawIdx::from(idx as u32)), krate))
    }

    /// Returns the path to the project's root folder.
    pub fn path(&self) -> &AbsPath {
        &self.project_root
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PolyJsonData {
    sysroot: Option<PathBuf>,
    sysroot_src: Option<PathBuf>,
    crates: Vec<PolyJsonCrateData>,
}

#[derive(Deserialize, Debug, Clone)]
struct PolyJsonCrateData {
    display_name: Option<String>,
    root_module: PathBuf,
    edition: EditionData,
    #[serde(default)]
    version: Option<semver::Version>,
    deps: Vec<PolyJsonDepData>,
    #[serde(default)]
    cfg: Vec<CfgFlag>,
    target: Option<String>,
    #[serde(default)]
    env: FxHashMap<String, String>,
    proc_macro_dylib_path: Option<PathBuf>,
    is_workspace_member: Option<bool>,
    source: Option<PolyJsonCrateSource>,
    #[serde(default)]
    is_proc_macro: bool,
    #[serde(default)]
    repository: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename = "edition")]
enum EditionData {
    #[serde(rename = "2015")]
    Edition2015,
    #[serde(rename = "2018")]
    Edition2018,
    #[serde(rename = "2021")]
    Edition2021,
}

impl From<EditionData> for Edition {
    fn from(data: EditionData) -> Self {
        match data {
            EditionData::Edition2015 => Edition::Edition2015,
            EditionData::Edition2018 => Edition::Edition2018,
            EditionData::Edition2021 => Edition::Edition2021,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct PolyJsonDepData {
    /// Identifies a crate by position in the crates array.
    #[serde(rename = "crate")]
    krate: usize,
    #[serde(deserialize_with = "deserialize_crate_name")]
    name: CrateName,
}

#[derive(Deserialize, Debug, Clone)]
struct PolyJsonCrateSource {
    include_dirs: Vec<PathBuf>,
    exclude_dirs: Vec<PathBuf>,
}

fn deserialize_crate_name<'de, D>(de: D) -> std::result::Result<CrateName, D::Error>
where
    D: de::Deserializer<'de>,
{
    let name = String::deserialize(de)?;
    CrateName::new(&name).map_err(|err| de::Error::custom(format!("invalid crate name: {err:?}")))
}
