//! Bounded constant-only receipts never consume the native renderer's work.
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

pub(crate) struct Startup {
    file: File,
    polled: bool,
    mounted: bool,
    pending: bool,
}

impl Startup {
    pub(crate) fn create(data: &Path) -> io::Result<Self> {
        let mut startup = Self {
            file: File::create(data.join("journey-startup.txt"))?,
            polled: false,
            mounted: false,
            pending: false,
        };
        startup.record("factory-entered")?;
        Ok(startup)
    }

    pub(crate) fn record(&mut self, phase: &str) -> io::Result<()> {
        writeln!(self.file, "{phase}")
    }

    pub(crate) fn observe(&mut self, mounted: bool, pending: bool) -> io::Result<()> {
        for (seen, reached, phase) in [
            (&mut self.polled, true, "service-polled"),
            (&mut self.mounted, mounted, "root-mounted"),
            (&mut self.pending, pending, "render-pending"),
        ] {
            if reached && !*seen {
                writeln!(self.file, "{phase}")?;
                *seen = true;
            }
        }

        Ok(())
    }
}
