use std::{fmt::Debug, fs::File, path::Path, process::Command};

use tempfile::TempDir;

pub struct Pipe {
    temp_dir: TempDir,
    file: File,
}

impl Pipe {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let full_path = temp_dir.path().join("pipe");
        make_pipe(&full_path);

        let mut file = File::options();
        let file = file.write(true).read(true).open(&full_path).unwrap();

        Pipe { temp_dir, file }
    }

    pub fn open(&self) -> &File {
        &self.file
    }

    pub fn directory(&self) -> &Path {
        self.temp_dir.path()
    }

    pub fn name(&self) -> &str {
        "pipe"
    }
}

impl Drop for Pipe {
    fn drop(&mut self) {
        log::trace!(
            "Dropping temporary folder {}",
            self.temp_dir.path().display()
        );
    }
}

impl Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pipe")
            .field("temp_dir", &self.temp_dir)
            .field("name", &self.name())
            .finish()
    }
}

#[derive(Debug)]
pub struct DuplexPipeConnection {
    to: Pipe,
    from: Pipe,
}

impl DuplexPipeConnection {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            to: Pipe::new(),
            from: Pipe::new(),
        }
    }
}

impl DuplexPipeConnection {
    pub fn from(&self) -> &Pipe {
        &self.from
    }

    pub fn to(&self) -> &Pipe {
        &self.to
    }
}

fn make_pipe(path: &Path) {
    log::trace!("Creating pipe at {}", path.display());

    let mut command = Command::new("mkfifo");
    let command = command.arg(path.display().to_string());

    assert!(command.spawn().unwrap().wait().unwrap().success());
}
