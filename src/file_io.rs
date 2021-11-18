use crate::prelude::*;

use macroquad::file;

pub async fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();
    let res = file::load_file(&path.to_string_lossy()).await?;
    Ok(res)
}

pub async fn load_file_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    let res = file::load_string(&path.to_string_lossy()).await?;
    Ok(res)
}
