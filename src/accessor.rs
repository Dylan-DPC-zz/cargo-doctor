use cargo_metadata::metadata;
use failure::Error;
use std::env::current_dir;
use std::path::{Path, PathBuf};

fn search(dir: &Path) -> Result<PathBuf, Error> {
    let manifest = dir.join("Cargo.toml");

    if metadata(Some(&manifest)).is_ok() {
        Ok(manifest)
    } else {
        search(dir.parent().expect("cannot access parent"))
    }
}

pub fn remote_path() -> Result<String, Error> {
    let current_crate = CurrentCrate::load()?;
    Ok(format!(
        "https://docs.rs/{}/{}/{}/",
        &current_crate.name, &current_crate.version, &current_crate.name
    ))
    //    Ok("https://docs.rs/uuid/0.7.0/uuid/".to_owned())
}

pub fn local_path() -> Result<String, Error> {
    let dir = current_dir()?;
    let current_crate = CurrentCrate::load()?;
    let path = dir.join(format!("target/doc/{}/", &current_crate.name));

    Ok(path.to_str().expect("local doc path is invalid").to_owned())
}

pub struct CurrentCrate {
    pub name: String,
    pub version: String,
}

impl CurrentCrate {
    fn load() -> Result<CurrentCrate, Error> {
        let manifest = search(&current_dir()?)?;
        let metadata = metadata(Some(manifest.as_path())).expect("can't read Cargo.toml");
        let metadata = &*(metadata.packages)
            .first()
            .expect("cannot find crate data from Cargo.toml");
            let name = metadata.name.replace("-", "_").to_owned();

        Ok(CurrentCrate {
            name,
            version: metadata.version.to_owned(),
        })
    }
}
