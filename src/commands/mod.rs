use std::{collections::HashMap, error::Error, fs::File};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct SyncCommand {}

impl SyncCommand {
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        // TODO(shelbyd): Get root from VCS.
        let root = ".";

        for result in ignore::Walk::new(root) {
            let entry = result?;
            let path = entry.path().strip_prefix("./")?;

            if path.file_name().and_then(|f| f.to_str()) != Some("MONO") {
                continue;
            }
            log::info!("Found MONO file {}", path.display());

            let dir = path.parent().expect("should never be root");

            let monofile: crate::Monofile = serde_yaml::from_reader(File::open(path)?)?;

            let mut interpolation_vars = HashMap::new();
            let dir_slug = dir.to_string_lossy().replace("/", "-");
            interpolation_vars.insert("dir_slug", dir_slug);

            for sync in &monofile.sync_files {
                for entry in glob::glob(&dir.join(&sync.from.0).display().to_string())? {
                    let from = entry?;
                    interpolation_vars.insert(
                        "file_name",
                        from.file_name().unwrap().to_string_lossy().to_string(),
                    );

                    let to = sync.to.interpolate(&interpolation_vars)?;
                    let to = if to.starts_with("/") {
                        format!("{}/{}", root, to.trim_start_matches("/"))
                    } else {
                        format!("{}/{}", dir.display(), to)
                    };

                    log::info!("Copying {} to {}", from.display(), &to);
                    let contents = std::fs::read_to_string(&from)?;
                    std::fs::write(&to, contents)?;
                }
                interpolation_vars.remove("file_name");
            }
        }

        Ok(())
    }
}
