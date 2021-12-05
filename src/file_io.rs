use crate::prelude::*;

use macroquad::file;
use serde::de::DeserializeOwned;

use crate::helpers::ToStringHelper;
use crate::json::Error;

pub async fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();
    let res = file::load_file(&path.to_string_helper()).await?;
    Ok(res)
}

pub async fn load_file_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    let res = file::load_string(&path.to_string_helper()).await?;
    Ok(res)
}

pub async fn deserialize_file<T, P: AsRef<Path>>(path: P) -> Result<T>
    where
        T: DeserializeOwned, {
    let path = path.as_ref();
    let path_str = path.to_string_helper();

    let bytes = load_file(&path_str).await?;
    match serde_json::from_slice(&bytes) {
        Err(err) => {
            return Err(Error::new(&path_str, err).into());
        }
        Ok(res) => {
            return Ok(res);
        }
    }
}