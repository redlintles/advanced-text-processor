use std::{ path::Path };

use crate::utils::errors::{ AtpError, AtpErrorCode };

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

pub fn check_chunk_bound_indexes(
    start_index: usize,
    end_index: usize,
    check_against: Option<&str>
) -> Result<(), AtpError> {
    match check_against {
        Some(text) => {
            if !(0..text.chars().count()).contains(&start_index) {
                return Err(
                    AtpError::new(
                        AtpErrorCode::IndexOutOfRange(
                            "Start index does not exist in current string!".to_string()
                        ),
                        "check_chunk_bound_indexes".to_string(),
                        text.to_string()
                    )
                );
            }

            return Ok(());
        }
        None => {}
    }
    if start_index >= end_index {
        return Err(
            AtpError::new(
                AtpErrorCode::InvalidIndex(
                    "Start index must be smaller than end index".to_string()
                ),
                format!("dlc {} {};", start_index, end_index),
                format!("Start Index: {}, End Index: {}", start_index, end_index)
            )
        );
    }

    Ok(())
}

pub fn check_index_against_input(index: usize, input: &str) -> Result<(), AtpError> {
    let character_count = input.chars().count();
    if !(0..character_count).contains(&index) {
        return Err(
            AtpError::new(
                AtpErrorCode::IndexOutOfRange(
                    format!(
                        "Index {} does not exist for {}, only indexes between 0-{} are allowed!",
                        index,
                        input,
                        character_count.saturating_sub(1)
                    )
                ),
                "Check index against input".to_string(),
                input.to_string()
            )
        );
    }

    Ok(())
}

pub fn check_vec_len(v: Vec<String>, max_size: usize) -> Result<(), AtpError> {
    match v.len() == max_size {
        true => Ok(()),
        false =>
            Err(
                AtpError::new(
                    AtpErrorCode::InvalidArgumentNumber(
                        format!(
                            "Only {} arguments are allowed for this instruction, passed {}",
                            max_size,
                            v.len()
                        )
                    ),
                    "check_vec_len".to_string(),
                    v.join(" ")
                )
            ),
    }
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod validations_tests {
    mod check_chunk_bound_indexes_tests {
        use crate::utils::validations::check_chunk_bound_indexes;
        #[test]
        fn success_with_no_reference_to_compare() {
            assert!(
                matches!(check_chunk_bound_indexes(1, 5, None), Ok(_)),
                "It does not throws an error for valid bounds"
            );
        }
        #[test]
        fn success_with_reference_to_compare() {
            assert!(
                matches!(check_chunk_bound_indexes(1, 5, Some("Banana")), Ok(_)),
                "It does not throws an error for valid bounds"
            );
        }
        #[test]
        fn error_with_no_reference_to_compare() {
            assert!(
                matches!(check_chunk_bound_indexes(2, 1, None), Err(_)),
                "It does throws an error for invalid bounds"
            );
        }
        #[test]
        fn error_with_reference_to_compare() {
            assert!(
                matches!(check_chunk_bound_indexes(10, 20, Some("Banana")), Err(_)),
                "It does throws an error for invalid start_index"
            );
        }
    }

    mod check_vec_len_tests {
        use crate::utils::validations::check_vec_len;

        #[test]
        fn success_valid_vec() {
            assert!(
                matches!(check_vec_len(["rtl".to_string(), (5).to_string()].to_vec(), 2), Ok(_))
            );
        }

        #[test]
        fn error_invalid_vec() {
            assert!(
                matches!(check_vec_len(["tls".to_string(), (5).to_string()].to_vec(), 1), Err(_))
            );
        }
    }

    mod check_index_against_input_tests {
        use crate::utils::validations::check_index_against_input;

        #[test]
        fn success_valid_index() {
            assert!(matches!(check_index_against_input(1, "Bánáná"), Ok(_)));
        }
        #[test]
        fn error_invalid_index() {
            assert!(matches!(check_index_against_input(7, "Bánáná"), Err(_)));
        }
    }
}
