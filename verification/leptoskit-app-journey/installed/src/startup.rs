//! Retain only bounded public phase names before deleting owned application data.
use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

const LIMIT: u64 = 1024;
const PHASES: [&str; 6] = [
    "factory-entered",
    "configuration-ready",
    "service-activation-entered",
    "service-polled",
    "root-mounted",
    "render-pending",
];

#[cfg(test)]
#[path = "../../runtime/src/startup.rs"]
mod runtime_startup;

#[cfg(test)]
#[path = "startup_tests.rs"]
mod tests;

pub(crate) fn retain(data: &Path, directory: &Path) -> io::Result<()> {
    let destination = directory.join("journey-startup.txt");

    let file = match File::open(data.join("journey-startup.txt")) {
        Ok(file) => file,

        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return fs::write(destination, b"receipt-missing-in-controller-data\n");
        }
        Err(error) => return Err(error),
    };

    let mut bytes = Vec::new();
    file.take(LIMIT + 1).read_to_end(&mut bytes)?;
    let text = std::str::from_utf8(&bytes).map_err(io::Error::other)?;

    if bytes.is_empty()
        || bytes.len() as u64 > LIMIT
        || text.lines().any(|line| !PHASES.contains(&line))
    {
        return Err(io::Error::other("invalid startup receipt"));
    }

    fs::write(destination, bytes)
}
