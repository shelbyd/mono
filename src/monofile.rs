use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Monofile {
    #[serde(default)]
    pub sync_files: Vec<SyncFiles>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SyncFiles {
    pub from: Glob,
    pub to: crate::InterpolatedString,
}

#[derive(Deserialize, Debug)]
pub struct Glob(pub PathBuf);
