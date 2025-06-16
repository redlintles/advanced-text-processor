use std::{ path::Path };

use crate::utils::errors::AtpError;

pub fn check_file_path(path: &Path, ext: Option<&str>) -> Result<(), AtpError> {
    let parsed_ext = ext.unwrap_or("atp");

    let mut v1: String = match path.extension() {
        Some(os_ext) => {
            let parsed_os_ext = os_ext.to_str().unwrap();
            if parsed_os_ext.to_string() == parsed_ext.to_string() {
                "".to_string()
            } else {
                format!("Path does not match required extension {}", parsed_ext)
            }
        }
        None => "Unable to get file extension".to_string(),
    };

    let v2: String = match path.parent() {
        Some(x) => {
            if x.exists() && !path.is_dir() {
                println!("LOGGING : {},{}", x.exists(), !path.is_dir());
                "".to_string()
            } else {
                "Parent should be an already existing directory".to_string()
            }
        }
        None => "Parent does not exists".to_string(),
    };

    v1.push_str(&v2);

    if v1.is_empty() {
        Ok(())
    } else {
        Err(
            AtpError::new(
                super::errors::AtpErrorCode::ValidationError("Validation Failed".to_string()),
                "".to_string(),
                path
                    .to_str()
                    .ok_or_else(||
                        AtpError::new(
                            super::errors::AtpErrorCode::ValidationError(
                                "Failed converting Path to string".to_string()
                            ),
                            "Path.to_str()".to_string(),
                            format!("{:?}", path).to_string()
                        )
                    )?
                    .to_string()
            )
        )
    }
}
