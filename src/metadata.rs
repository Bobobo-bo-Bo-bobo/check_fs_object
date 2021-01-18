use std::error::Error;
use std::fs;

pub fn get_metadata(obj: &str, follow: bool) -> Result<std::fs::Metadata, Box<dyn Error>> {
    let meta: fs::Metadata;

    if follow {
        meta = fs::metadata(obj)?;
    } else {
        meta = fs::symlink_metadata(obj)?;
    }

    Ok(meta)
}

