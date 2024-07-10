use std::fs::OpenOptions;
use std::path::Path;

fn read_write_create_and_open_file(path: &Path) -> std::io::Result<()> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;

    Ok(())
}
