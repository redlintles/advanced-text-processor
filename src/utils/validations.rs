use std::{ borrow::Cow, path::Path };

use crate::utils::errors::{ AtpError, AtpErrorCode };

pub fn check_file_path(path: &Path, ext: Option<&str>) -> Result<(), AtpError> {
    let parsed_ext = ext.unwrap_or("atp");

    // canonicalize() exige que o path exista; isso é ok aqui porque queremos validar um arquivo real.
    let path = path
        .canonicalize()
        .map_err(|e| {
            AtpError::new(
                AtpErrorCode::ValidationError("Path canonicalization failed".into()),
                Cow::Borrowed("canonicalize"),
                format!("{:?} - {}", path, e)
            )
        })?;

    // Extensão
    let os_ext = path
        .extension()
        .and_then(|x| x.to_str())
        .ok_or_else(|| {
            AtpError::new(
                AtpErrorCode::ValidationError("No file extension found".into()),
                Cow::Borrowed("check_file_path"),
                path.to_string_lossy().to_string()
            )
        })?;

    if os_ext != parsed_ext {
        return Err(
            AtpError::new(
                AtpErrorCode::ValidationError("Wrong file extension".into()),
                Cow::Borrowed("check_file_path"),
                path.to_string_lossy().to_string()
            )
        );
    }

    // Diretório pai (redundante depois do canonicalize, mas mantém a intenção explícita)
    let parent = path
        .parent()
        .ok_or_else(|| {
            AtpError::new(
                AtpErrorCode::ValidationError("Path has no parent directory".into()),
                Cow::Borrowed("check_file_path"),
                path.to_string_lossy().to_string()
            )
        })?;

    if !parent.exists() {
        return Err(
            AtpError::new(
                AtpErrorCode::ValidationError("Parent directory does not exist".into()),
                Cow::Borrowed("check_file_path"),
                parent.to_string_lossy().to_string()
            )
        );
    }

    if path.is_dir() {
        return Err(
            AtpError::new(
                AtpErrorCode::ValidationError("Path is a directory, not a file".into()),
                Cow::Borrowed("check_file_path"),
                path.to_string_lossy().to_string()
            )
        );
    }

    Ok(())
}

/// Valida limites de chunk com índices em *contagem de chars* (não bytes),
/// assumindo semântica inclusiva em tokens (start..=end).
///
/// Regras:
/// - Sempre exige `start_index < end_index`
/// - Se `check_against` for Some(text):
///     - exige `start_index` e `end_index` dentro de `0..text.chars().count()`
///
/// Obs: se você quiser permitir chunks de 1 caractere (start==end), mude a regra.
pub fn check_chunk_bound_indexes(
    start_index: usize,
    end_index: usize,
    check_against: Option<&str>
) -> Result<(), AtpError> {
    // regra estrutural (independente do texto)
    if start_index >= end_index {
        let fmt_err = format!("check_chunk_bound_indexes {} {};", start_index, end_index);
        return Err(
            AtpError::new(
                AtpErrorCode::InvalidIndex("Start index must be smaller than end index".into()),
                Cow::Owned(fmt_err),
                format!("Start Index: {}, End Index: {}", start_index, end_index)
            )
        );
    }

    if let Some(text) = check_against {
        let total_chars = text.chars().count();

        // start precisa existir
        if !(0..total_chars).contains(&start_index) {
            return Err(
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        "Start index does not exist in current string!".into()
                    ),
                    Cow::Borrowed("check_chunk_bound_indexes"),
                    text.to_string()
                )
            );
        }

        // end também precisa existir
        if !(0..total_chars).contains(&end_index) {
            return Err(
                AtpError::new(
                    AtpErrorCode::IndexOutOfRange(
                        "End index does not exist in current string!".into()
                    ),
                    Cow::Borrowed("check_chunk_bound_indexes"),
                    text.to_string()
                )
            );
        }
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
                    ).into()
                ),
                Cow::Borrowed("check_index_against_input"),
                input.to_string()
            )
        );
    }

    Ok(())
}

pub fn check_index_against_words(index: usize, input: &str) -> Result<(), AtpError> {
    let word_count = input.split_whitespace().count();

    if word_count == 0 {
        return Err(
            AtpError::new(
                AtpErrorCode::IndexOutOfRange("Input has no words".into()),
                Cow::Borrowed("check_index_against_words"),
                input.to_string()
            )
        );
    }

    if !(0..word_count).contains(&index) {
        return Err(
            AtpError::new(
                AtpErrorCode::IndexOutOfRange(
                    format!(
                        "Word index {} does not exist for input, only indexes between 0-{} are allowed!",
                        index,
                        word_count.saturating_sub(1)
                    ).into()
                ),
                Cow::Borrowed("check_index_against_words"),
                input.to_string()
            )
        );
    }

    Ok(())
}

pub fn check_vec_len(v: &Vec<String>, max_size: usize) -> Result<(), AtpError> {
    if v.len() == max_size {
        Ok(())
    } else {
        Err(
            AtpError::new(
                AtpErrorCode::InvalidArgumentNumber(
                    format!(
                        "Only {} arguments are allowed for this instruction, passed {}",
                        max_size,
                        v.len()
                    ).into()
                ),
                Cow::Borrowed("check_vec_len"),
                v.join(" ")
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ fs, time::{ SystemTime, UNIX_EPOCH } };

    fn unique_tmp_dir(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let mut p = std::env::temp_dir();
        p.push(format!("{}_{}_{}", prefix, std::process::id(), nanos));
        p
    }

    mod check_file_path_tests {
        use super::*;

        #[test]
        fn ok_for_existing_file_with_default_ext_atp() {
            let dir = unique_tmp_dir("atp_check_file_path_ok");
            fs::create_dir_all(&dir).unwrap();

            let file = dir.join("input.atp");
            fs::write(&file, "banana").unwrap();

            assert!(check_file_path(&file, None).is_ok());

            let _ = fs::remove_dir_all(&dir);
        }

        #[test]
        fn ok_for_existing_file_with_custom_ext() {
            let dir = unique_tmp_dir("atp_check_file_path_custom_ext");
            fs::create_dir_all(&dir).unwrap();

            let file = dir.join("input.atpbc");
            fs::write(&file, "banana").unwrap();

            assert!(check_file_path(&file, Some("atpbc")).is_ok());

            let _ = fs::remove_dir_all(&dir);
        }

        #[test]
        fn err_for_wrong_extension() {
            let dir = unique_tmp_dir("atp_check_file_path_wrong_ext");
            fs::create_dir_all(&dir).unwrap();

            let file = dir.join("input.txt");
            fs::write(&file, "banana").unwrap();

            let r = check_file_path(&file, None);
            assert!(r.is_err());

            let _ = fs::remove_dir_all(&dir);
        }

        #[test]
        fn err_for_missing_extension() {
            let dir = unique_tmp_dir("atp_check_file_path_no_ext");
            fs::create_dir_all(&dir).unwrap();

            let file = dir.join("input");
            fs::write(&file, "banana").unwrap();

            let r = check_file_path(&file, None);
            assert!(r.is_err());

            let _ = fs::remove_dir_all(&dir);
        }

        #[test]
        fn err_for_directory_path() {
            let dir = unique_tmp_dir("atp_check_file_path_is_dir");
            fs::create_dir_all(&dir).unwrap();

            // canonicalize vai funcionar, mas is_dir deve falhar
            let r = check_file_path(&dir, None);
            assert!(r.is_err());

            let _ = fs::remove_dir_all(&dir);
        }

        #[test]
        fn err_for_nonexistent_path_canonicalize_fails() {
            let dir = unique_tmp_dir("atp_check_file_path_missing");
            // não cria diretório/arquivo
            let file = dir.join("missing.atp");

            let r = check_file_path(&file, None);
            assert!(r.is_err());
        }
    }

    mod check_chunk_bound_indexes_tests {
        use super::*;

        #[test]
        fn ok_when_start_lt_end_and_no_text() {
            assert!(check_chunk_bound_indexes(1, 3, None).is_ok());
        }

        #[test]
        fn err_when_start_ge_end_no_text() {
            assert!(check_chunk_bound_indexes(3, 3, None).is_err());
            assert!(check_chunk_bound_indexes(4, 2, None).is_err());
        }

        #[test]
        fn ok_when_indices_in_range_with_text() {
            // "banàna" tem 6 chars
            let text = "banàna";
            assert!(check_chunk_bound_indexes(1, 4, Some(text)).is_ok());
        }

        #[test]
        fn err_when_start_out_of_range_with_text() {
            let text = "abc"; // 3 chars: idx 0..2
            assert!(check_chunk_bound_indexes(3, 4, Some(text)).is_err());
        }

        #[test]
        fn err_when_end_out_of_range_with_text() {
            let text = "abc"; // 3 chars: idx 0..2
            assert!(check_chunk_bound_indexes(1, 3, Some(text)).is_err());
        }

        #[test]
        fn err_when_start_ge_end_with_text() {
            let text = "abcdef";
            assert!(check_chunk_bound_indexes(2, 2, Some(text)).is_err());
            assert!(check_chunk_bound_indexes(3, 1, Some(text)).is_err());
        }
    }

    mod check_index_against_input_tests {
        use super::*;

        #[test]
        fn ok_when_index_in_range() {
            assert!(check_index_against_input(0, "banana").is_ok());
            assert!(check_index_against_input(5, "banana").is_ok());
        }

        #[test]
        fn err_when_index_out_of_range() {
            assert!(check_index_against_input(6, "banana").is_err());
            assert!(check_index_against_input(999, "banana").is_err());
        }

        #[test]
        fn err_when_input_empty() {
            assert!(check_index_against_input(0, "").is_err());
        }
    }

    mod check_vec_len_tests {
        use super::*;

        #[test]
        fn ok_when_len_matches() {
            let v = vec!["a".to_string(), "b".to_string()];
            assert!(check_vec_len(&v, 2).is_ok());
        }

        #[test]
        fn err_when_len_does_not_match() {
            let v = vec!["a".to_string(), "b".to_string()];
            assert!(check_vec_len(&v, 1).is_err());
            assert!(check_vec_len(&v, 3).is_err());
        }

        #[test]
        fn ok_for_empty_vec_when_expected_zero() {
            let v: Vec<String> = vec![];
            assert!(check_vec_len(&v, 0).is_ok());
        }
    }

    mod check_index_against_words_tests {
        use super::*;

        #[test]
        fn ok_when_index_in_range() {
            assert!(check_index_against_words(0, "banana laranja").is_ok());
            assert!(check_index_against_words(1, "banana laranja").is_ok());
        }

        #[test]
        fn ok_with_multiple_spaces_and_newlines() {
            assert!(check_index_against_words(2, "banana   laranja\ncheia\tde  canja").is_ok());
        }

        #[test]
        fn err_when_index_out_of_range() {
            assert!(check_index_against_words(2, "banana laranja").is_err());
            assert!(check_index_against_words(999, "banana laranja").is_err());
        }

        #[test]
        fn err_when_input_has_no_words() {
            assert!(check_index_against_words(0, "").is_err());
            assert!(check_index_against_words(0, "     \n\t  ").is_err());
        }

        #[test]
        fn supports_unicode_words() {
            // split_whitespace funciona bem com unicode; aqui garante que o contador não quebra
            assert!(check_index_against_words(1, "banàna maçã").is_ok());
            assert!(check_index_against_words(2, "banàna maçã").is_err());
        }
    }

    // aqui fica seu padrão "bytecode_tests" (não há bytecode nessas funcs, mas mantive o esqueleto)
    mod bytecode_tests {
        // sem testes: utilitários não geram bytecode
        #[test]
        fn placeholder() {
            assert!(true);
        }
    }
}
