//! Only a directory this run atomically created can be removed by its cleanup.
use crate::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) struct ApplicationData(PathBuf);

impl ApplicationData {
    pub(crate) fn create(path: PathBuf) -> Result<Self> {
        fs::create_dir(&path)?;
        Ok(Self(path))
    }

    pub(crate) fn path(&self) -> &Path {
        &self.0
    }

    pub(crate) fn close(&self) -> Result<()> {
        match fs::remove_dir_all(&self.0) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            result => result.map_err(Into::into),
        }
    }
}

impl Drop for ApplicationData {
    fn drop(&mut self) {
        if let Err(error) = self.close() {
            eprintln!("Owned application data cleanup failed: {error}");
        }
    }
}
