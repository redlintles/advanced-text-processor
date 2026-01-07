use std::collections::HashMap;
use std::path::Path;

use uuid::Uuid;

use colored::*;

use crate::builder::atp_builder::AtpBuilder;
#[cfg(feature = "bytecode")]
use crate::bytecode::{ reader::read_bytecode_from_file, writer::write_bytecode_to_file };
use crate::context::execution_context::GlobalExecutionContext;
use crate::tokens::TokenMethods;

use crate::text::parser::parse_token;
use crate::text::reader::read_from_file;
use crate::text::writer::write_to_file;

use crate::utils::errors::{ AtpError, ErrorManager, token_array_not_found };

/// ATP Processor
///
/// `AtpProcessor` is the main **execution engine** of ATP (Advanced Text Processor).
///
/// It stores multiple linear transformation pipelines (called **transforms**) identified
/// by a `String` ID (generated with UUID). Each transform is a `Vec<Box<dyn TokenMethods>>`,
/// i.e. a sequence of tokens executed from left to right.
///
/// # Core concepts
///
/// ## Transform (pipeline)
/// A *transform* is a vector of tokens executed sequentially:
///
/// 1. Start with `result = input`
/// 2. For each token in order: `result = token.transform(result)`
/// 3. Return the final `result`
///
/// This matches ATP's **linear** nature: there is no implicit nesting at runtime.
/// Any logical grouping/abstraction lives at the builder / authoring layer.
///
/// ## Error accumulation (WIP)
/// `AtpProcessor` contains an `ErrorManager` meant to accumulate errors found during
/// reading/writing/execution. At the moment, `ErrorManager` is still under construction;
/// therefore, this should be treated as an internal error sink rather than a stable public
/// diagnostics API.
///
/// # Examples
///
/// ## 1) Build a pipeline through the processor, then run it
///
/// ```rust
/// use atp::builder::atp_processor::{AtpProcessor, AtpProcessorMethods};
/// use atp::builder::AtpBuilderMethods;
///
/// let mut processor = AtpProcessor::new();
///
/// // Build + register a transform; `build()` returns its ID.
/// let id = processor
///     .create_pipeline()
///     .trim_both_sides()?
///     .add_to_end("!")?
///     .build();
///
/// let out = processor.process_all(&id, "   banana   ")?;
/// assert_eq!(out, "banana!");
/// # Ok::<(), atp::utils::errors::AtpError>(())
/// ```
///
/// ## 2) Step-by-step debug execution (SBS)
///
/// ```rust
/// use atp::builder::atp_processor::{AtpProcessor, AtpProcessorMethods};
/// use atp::builder::AtpBuilderMethods;
///
/// let mut processor = AtpProcessor::new();
///
/// let id = processor
///     .create_pipeline()
///     .add_to_beginning("Banana")?
///     .add_to_end("pizza")?
///     .repeat(3)?
///     .trim_both_sides()?
///     .build();
///
/// // Prints each step: instruction + before/after.
/// let out = processor.process_all_with_debug(&id, "Banana Laranja cheia de canja")?;
/// println!("{out}");
/// # Ok::<(), atp::utils::errors::AtpError>(())
/// ```
///
/// ## 3) Quick single-token execution (no pipeline registration)
///
/// ```rust
/// use atp::builder::atp_processor::{AtpProcessor, AtpProcessorMethods};
/// use atp::tokens::transforms::tbs;
///
/// let mut processor = AtpProcessor::new();
///
/// let token = Box::new(tbs::Tbs::default());
/// let out = processor.process_single(token, "   banana   ")?;
/// assert_eq!(out, "banana");
/// # Ok::<(), atp::utils::errors::AtpError>(())
/// ```
///
/// ## 4) Benchmark-style usage (as in your tests)
///
/// This mirrors the exact usage pattern shown in your suite:
///
/// ```rust
/// use atp::builder::{
///     AtpBuilderMethods,
///     atp_processor::{ AtpProcessor, AtpProcessorMethods },
/// };
/// use std::time::Instant;
///
/// # fn main() -> Result<(), atp::utils::errors::AtpError> {
/// let runs = 100;
/// let mut total = 0.0;
///
/// let mut processor = AtpProcessor::new();
///
/// let id = processor
///     .create_pipeline()
///     .add_to_beginning("Banana")?
///     .add_to_end("pizza")?
///     .repeat(3)?
///     .delete_after(20)?
///     .delete_before(3)?
///     .delete_chunk(0, 3)?
///     .delete_first()?
///     .delete_last()?
///     .replace_all_with("a", "e")?
///     .replace_first_with("L", "coxinha")?
///     .replace_count_with("e", "carro", 3)?
///     .insert(0, "Coxinha Banana")?
///     .rotate_left(1)?
///     .rotate_right(2)?
///     .trim_both_sides()?
///     .trim_left_side()?
///     .trim_right_side()?
///     .add_to_beginning("laranjadebananavermelha")?
///     .select(3, 7)?
///     .replace_count_with("a", "b", 3)?
///     .to_uppercase_all()?
///     .to_lowercase_all()?
///     .to_uppercase_single(3)?
///     .to_lowercase_single(2)?
///     .capitalize_first_word()?
///     .capitalize_single_word(1)?
///     .capitalize_last_word()?
///     .capitalize_range(1, 3)?
///     .split_select("B", 1)?
///     .capitalize_chunk(1, 3)?
///     .replace_last_with("b", "c")?
///     .replace_nth_with("b", "d", 3)?
///     .to_url_encoded()?
///     .to_url_decoded()?
///     .to_reverse()?
///     .split_characters()?
///     .to_html_escaped()?
///     .to_html_unescaped()?
///     .to_json_escaped()?
///     .to_json_unescaped()?
///     .insert(1, "banana")?
///     .to_uppercase_chunk(1, 3)?
///     .to_lowercase_chunk(0, 5)?
///     .join_to_camel_case()?
///     .join_to_kebab_case()?
///     .join_to_pascal_case()?
///     .join_to_snake_case()?
///     .pad_left("xy", 12)?
///     .pad_right("yx", 20)?
///     .build();
///
/// for _ in 0..runs {
///     let start = Instant::now();
///     let input = "Banana Laranja cheia de canja";
///     let _ = processor.process_all(&id, input)?;
///     total += start.elapsed().as_secs_f64();
/// }
///
/// let avg = total / runs as f64;
/// println!("Average: {avg:.6}s");
/// # Ok(())
/// # }
/// ```
///
/// # Notes
///
/// - `build()` registers a new transform entry inside the processor and returns its UUID.
/// - The pipeline is **one giant vector** of tokens; execution is deterministic and ordered.
/// - Debug methods (`*_with_debug`) only add printing; they do not change execution.
pub struct AtpProcessor {
    transforms: HashMap<String, Vec<Box<dyn TokenMethods>>>,
    errors: ErrorManager,
}

/// Operational API for `AtpProcessor`.
///
/// This trait defines the public “surface” of the processor: how pipelines are registered,
/// executed, persisted, inspected, and removed.
///
/// A **transform** is stored internally as:
/// `HashMap<String, Vec<Box<dyn TokenMethods>>>`
///
/// Where the key is a UUID string and the value is a linear sequence of tokens.
///
/// ## Error reporting
/// Most methods will:
/// - return `Err(AtpError)` on failure
/// - and also push a copy into the internal `ErrorManager` (where you already do that)
///
/// The exact behavior depends on the implementation (and your `ErrorManager` is still WIP).
pub trait AtpProcessorMethods {
    /// Writes a registered transform (pipeline) to an `.atp` text file.
    ///
    /// Internally:
    /// - looks up `id` in `self.transforms`
    /// - if found, calls `write_to_file(path, tokens)`
    /// - if not found, returns `TokenArrayNotFound` (via `token_array_not_found`)
    ///
    /// # Parameters
    /// - `id`: Transform identifier previously returned by `add_transform()` / `build()`.
    /// - `path`: Destination path to write the textual representation.
    ///
    /// # Errors
    /// Returns `Err` if:
    /// - the transform does not exist
    /// - writing fails (I/O or serialization problems inside `write_to_file`)
    fn write_to_text_file(&mut self, id: &str, path: &Path) -> Result<(), AtpError>;

    /// Reads an `.atp` text file, parses it into tokens, registers it as a new transform,
    /// and returns the newly created transform ID.
    ///
    /// Internally:
    /// - reads and parses tokens via `read_from_file(path)`
    /// - generates a new UUID
    /// - inserts the parsed vector into `self.transforms`
    ///
    /// # Returns
    /// The UUID string identifying the newly registered transform.
    ///
    /// # Errors
    /// Returns `Err` if reading/parsing the file fails.
    fn read_from_text_file(&mut self, path: &Path) -> Result<String, AtpError>;

    /// Registers a new transform (pipeline) directly from a token vector.
    ///
    /// This is the low-level “insert” API. Higher-level builder APIs typically call this.
    ///
    /// Internally:
    /// - generates a new UUID
    /// - inserts `(uuid -> tokens)` into `self.transforms`
    ///
    /// # Returns
    /// The UUID string identifying the newly registered transform.
    fn add_transform(&mut self, tokens: Vec<Box<dyn TokenMethods>>) -> String;

    /// Executes all tokens of a registered transform from left to right.
    ///
    /// Semantics:
    /// - `result` starts as `input`
    /// - for each token `t` in the transform:
    ///   - `result = parse_token(t, result, &mut self.errors)?`
    /// - returns the final `result`
    ///
    /// `parse_token` is used instead of calling `token.transform` directly because it can
    /// integrate with your parsing/diagnostics/error flow (and will likely be where conditional
    /// execution, blocks, etc. plug in later).
    ///
    /// # Parameters
    /// - `id`: Transform identifier.
    /// - `input`: Input string to process.
    ///
    /// # Errors
    /// Returns `Err` if:
    /// - the transform does not exist
    /// - any token execution fails (propagated from `parse_token`)
    fn process_all(&mut self, id: &str, input: &str) -> Result<String, AtpError>;

    /// Executes a single token over `input`, without registering it into the processor.
    ///
    /// This is a convenience method for ad-hoc transformations:
    /// - calls `token.transform(input)`
    /// - stores any encountered error in the internal error manager (in your impl)
    ///
    /// # Parameters
    /// - `token`: The token to execute once.
    /// - `input`: Input string.
    ///
    /// # Errors
    /// Returns `Err` if the token’s `transform` fails.
    fn process_single(
        &mut self,
        token: Box<dyn TokenMethods>,
        input: &str
    ) -> Result<String, AtpError>;

    /// Executes a registered transform like `process_all`, but prints each step.
    ///
    /// Intended for debugging and teaching:
    /// - prints an SBS header
    /// - for each token:
    ///   - computes `temp = parse_token(...)`
    ///   - prints: step index, instruction (`to_atp_line()`), before, after
    /// - returns the final result
    ///
    /// # Parameters
    /// - `id`: Transform identifier.
    /// - `input`: Input string.
    ///
    /// # Errors
    /// Same error behavior as `process_all`.
    fn process_all_with_debug(&mut self, id: &str, input: &str) -> Result<String, AtpError>;

    /// Executes a single token like `process_single`, but prints a single SBS step.
    ///
    /// Intended for debugging token behavior in isolation.
    ///
    /// Prints:
    /// - Step 0 -> 1
    /// - Instruction (`to_atp_line()`)
    /// - Before / After
    ///
    /// # Errors
    /// Returns `Err` if the token’s `transform` fails.
    fn process_single_with_debug(
        &mut self,
        token: Box<dyn TokenMethods>,
        input: &str
    ) -> Result<String, AtpError>;

    /// Removes a transform from the processor.
    ///
    /// Internally:
    /// - performs `self.transforms.remove(id)`
    /// - returns `Ok(())` if something was removed
    /// - returns `Err(TokenNotFound)` (your custom error) if the ID does not exist
    ///
    /// # Errors
    /// Returns `Err` if the transform does not exist.
    fn remove_transform(&mut self, id: &str) -> Result<(), AtpError>;

    /// Displays the list of registered transform IDs.
    ///
    /// Your current implementation prints:
    /// - an index counter
    /// - the UUID key
    ///
    /// # Note
    /// This is pure side-effect (stdout). It does not return the data.
    ///
    /// The trait provides a default empty body `{}` so implementors may override it.
    fn show_transforms(&self) -> () {}

    /// Checks whether a transform with the given `id` exists.
    ///
    /// Internally: `self.transforms.contains_key(id)`
    fn transform_exists(&self, id: &str) -> bool;

    /// Returns a **cloned** copy of the token vector for a given transform `id`.
    ///
    /// This method is useful for:
    /// - inspection
    /// - exporting
    /// - composing transforms (if you later support merging)
    ///
    /// # Returns
    /// A cloned `Vec<Box<dyn TokenMethods>>`.
    ///
    /// # Errors
    /// Returns `Err(TokenArrayNotFound)` if the transform does not exist.
    fn get_transform_vec(&self, id: &str) -> Result<Vec<Box<dyn TokenMethods>>, AtpError>;

    /// Returns the textual `.atp` lines for a given transform `id`.
    ///
    /// Internally:
    /// - clones the transform vector
    /// - maps each token to `token.to_atp_line().to_string()`
    ///
    /// This is typically what you want for:
    /// - UI display
    /// - exporting to text
    /// - debugging what the pipeline “looks like”
    ///
    /// # Errors
    /// Returns `Err(TokenArrayNotFound)` if the transform does not exist.
    fn get_text_transform_vec(&self, id: &str) -> Result<Vec<String>, AtpError>;

    /// Writes a registered transform to an ATP bytecode file (`.atpbc`).
    ///
    /// Available only with the `bytecode` feature.
    ///
    /// Internally:
    /// - looks up `id` in `self.transforms`
    /// - calls `write_bytecode_to_file(path, tokens.to_vec())`
    ///
    /// # Errors
    /// Returns `Err` if the transform does not exist or bytecode writing fails.
    #[cfg(feature = "bytecode")]
    fn write_to_bytecode_file(&mut self, id: &str, path: &Path) -> Result<(), AtpError>;

    /// Reads an ATP bytecode file (`.atpbc`), registers it as a new transform, and returns its ID.
    ///
    /// Available only with the `bytecode` feature.
    ///
    /// Internally:
    /// - parses tokens via `read_bytecode_from_file(path)`
    /// - registers them using `add_transform`
    ///
    /// # Returns
    /// The UUID string identifying the newly registered transform.
    ///
    /// # Errors
    /// Returns `Err` if bytecode reading/parsing fails.
    #[cfg(feature = "bytecode")]
    fn read_from_bytecode_file(&mut self, path: &Path) -> Result<String, AtpError>;

    /// Executes a registered transform like `process_all_with_debug`, but intended for
    /// bytecode-loaded transforms (you still execute tokens the same way).
    ///
    /// Available only with the `bytecode` feature.
    ///
    /// Internally (your impl):
    /// - looks up transform
    /// - iterates tokens and calls `parse_token(...)` while printing SBS output
    ///
    /// # Errors
    /// Same as `process_all_with_debug`.
    #[cfg(feature = "bytecode")]
    fn process_all_bytecode_with_debug(
        &mut self,
        id: &str,
        input: &str
    ) -> Result<String, AtpError>;

    /// Executes a single token like `process_single_with_debug`, but provided under the
    /// `bytecode` feature flag for API symmetry.
    ///
    /// Available only with the `bytecode` feature.
    ///
    /// # Errors
    /// Returns `Err` if the token’s `transform` fails.
    #[cfg(feature = "bytecode")]
    fn process_single_bytecode_with_debug(
        &mut self,
        token: Box<dyn TokenMethods>,
        input: &str
    ) -> Result<String, AtpError>;
}

impl AtpProcessor {
    /// Creates a new empty processor.
    ///
    /// - No transforms are registered initially.
    /// - The internal `ErrorManager` is initialized with `Default`.
    pub fn new() -> Self {
        AtpProcessor {
            transforms: HashMap::new(),
            errors: ErrorManager::default(),
        }
    }

    /// Creates an `AtpBuilder` bound to this processor.
    ///
    /// The builder accumulates tokens and, when `build()` is called, it registers a new
    /// transform entry inside this processor and returns the corresponding transform ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use atp::builder::atp_processor::{AtpProcessor, AtpProcessorMethods};
    /// use atp::builder::AtpBuilderMethods;
    ///
    /// let mut processor = AtpProcessor::new();
    ///
    /// let id = processor
    ///     .create_pipeline()
    ///     .trim_both_sides()?
    ///     .add_to_end("!")?
    ///     .build();
    ///
    /// let out = processor.process_all(&id, "   banana   ")?;
    /// assert_eq!(out, "banana!");
    /// # Ok::<(), atp::utils::errors::AtpError>(())
    /// ```
    pub fn create_pipeline(&mut self) -> AtpBuilder<'_> {
        AtpBuilder::new(self)
    }
}

impl AtpProcessorMethods for AtpProcessor {
    fn write_to_text_file(&mut self, id: &str, path: &Path) -> Result<(), AtpError> {
        let tokens = match self.transforms.get(id).ok_or_else(token_array_not_found(id)) {
            Ok(x) => x,
            Err(e) => {
                self.errors.add_error(e.clone());
                return Err(e);
            }
        };

        write_to_file(Path::new(path), tokens)
    }

    fn read_from_text_file(&mut self, path: &Path) -> Result<String, AtpError> {
        let tokens = match read_from_file(Path::new(path)) {
            Ok(x) => x,
            Err(e) => {
                self.errors.add_error(e.clone());
                return Err(e);
            }
        };

        let identifier = Uuid::new_v4();

        self.transforms.insert(identifier.to_string(), tokens);

        Ok(identifier.to_string())
    }

    fn process_all(&mut self, id: &str, input: &str) -> Result<String, AtpError> {
        let mut result = String::from(input);

        let tokens = self.transforms.get(id).ok_or_else(token_array_not_found(id));

        match tokens {
            Ok(tks) => {
                let mut context = GlobalExecutionContext::new();
                for token in tks.iter() {
                    result = parse_token(
                        token.as_ref(),
                        result.as_str(),
                        &mut self.errors,
                        &mut context
                    )?;
                }
                Ok(result.to_string())
            }
            Err(e) => {
                self.errors.add_error(e.clone());
                Err(e)
            }
        }
    }

    fn add_transform(&mut self, tokens: Vec<Box<dyn TokenMethods>>) -> String {
        let identifier = Uuid::new_v4();
        self.transforms.insert(identifier.to_string(), tokens);

        identifier.to_string()
    }

    fn remove_transform(&mut self, id: &str) -> Result<(), AtpError> {
        match
            self.transforms
                .remove(id)
                .ok_or_else(|| {
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::TokenNotFound(
                            "Transformation not found".into()
                        ),
                        "remove_transform",
                        id.to_string()
                    )
                })
        {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    fn show_transforms(&self) -> () {
        for (i, k) in self.transforms.keys().enumerate() {
            println!("{} - {}", i, k);
        }
    }

    fn transform_exists(&self, id: &str) -> bool {
        self.transforms.contains_key(id)
    }

    fn get_transform_vec(&self, id: &str) -> Result<Vec<Box<dyn TokenMethods>>, AtpError> {
        Ok(
            self.transforms
                .get(id)
                .ok_or_else(|| {
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::TokenArrayNotFound(
                            "Transform not found".into()
                        ),
                        "get_transform_vec".to_string(),
                        id.to_string()
                    )
                })?
                .clone()
        )
    }
    fn get_text_transform_vec(&self, id: &str) -> Result<Vec<String>, AtpError> {
        Ok(
            self.transforms
                .get(id)
                .ok_or_else(|| {
                    AtpError::new(
                        crate::utils::errors::AtpErrorCode::TokenArrayNotFound(
                            "Transform not found".into()
                        ),
                        "get_transform_vec",
                        id.to_string()
                    )
                })?
                .clone()
                .iter()
                .map(|t| t.to_atp_line().to_string())
                .collect::<Vec<String>>()
        )
    }

    fn process_single(
        &mut self,
        token: Box<dyn TokenMethods>,
        input: &str
    ) -> Result<String, AtpError> {
        match token.transform(input) {
            Ok(x) => Ok(x),
            Err(e) => {
                self.errors.add_error(e.clone());
                Err(e)
            }
        }
    }
    fn process_all_with_debug(&mut self, id: &str, input: &str) -> Result<String, AtpError> {
        let mut result = input.to_string();
        let dashes = 10;

        let tokens = match self.transforms.get(id).ok_or_else(token_array_not_found(id)) {
            Ok(x) => x,
            Err(e) => {
                self.errors.add_error(e.clone());
                return Err(e);
            }
        };

        let mut log = String::new();
        log.push_str("PROCESSING STEP BY STEP:\n");
        log.push_str(&"-".repeat(dashes));
        log.push_str("\n\n");

        let mut context = GlobalExecutionContext::new();

        for (counter, token) in (0_i64..).zip(tokens.iter()) {
            let temp = parse_token(
                token.as_ref(),
                result.as_str(),
                &mut self.errors,
                &mut context
            )?;

            // Note: format! aloca, mas agora você faz 1 print no final.
            log.push_str(
                &format!(
                    "Step: [{}] => [{}]\nInstruction: {}\nBefore: {}\nAfter: {}\n\n",
                    counter.to_string().blue(),
                    (counter + 1).to_string().blue(),
                    token.to_atp_line().yellow(),
                    result.red(),
                    temp.green()
                )
            );

            if (counter as usize) + 1 < tokens.len() {
                log.push_str(&"-".repeat(dashes));
                log.push_str("\n\n");
            }

            result = temp;
        }

        print!("{log}"); // 1 única saída
        Ok(result)
    }

    fn process_single_with_debug(
        &mut self,
        token: Box<dyn TokenMethods>,
        input: &str
    ) -> Result<String, AtpError> {
        let output = match token.transform(input) {
            Ok(x) => x,
            Err(e) => {
                self.errors.add_error(e.clone());
                return Err(e);
            }
        };
        println!(
            "Step: [{}] => [{}]\nInstruction: {}\nBefore: {}\nAfter: {}\n",
            (0).to_string().blue(),
            (1).to_string().blue(),
            token.to_atp_line().yellow(),
            input.red(),
            output.green()
        );

        Ok(output)
    }
    #[cfg(feature = "bytecode")]
    fn write_to_bytecode_file(&mut self, id: &str, path: &Path) -> Result<(), AtpError> {
        let tokens = match self.transforms.get(id).ok_or_else(token_array_not_found(id)) {
            Ok(x) => x,
            Err(e) => {
                self.errors.add_error(e.clone());
                return Err(e);
            }
        };

        write_bytecode_to_file(path, tokens.to_vec())
    }
    #[cfg(feature = "bytecode")]
    fn read_from_bytecode_file(&mut self, path: &Path) -> Result<String, AtpError> {
        let tokens = match read_bytecode_from_file(path) {
            Ok(x) => x,
            Err(e) => {
                self.errors.add_error(e.clone());
                return Err(e);
            }
        };

        let identifier = self.add_transform(tokens.to_vec());

        Ok(identifier)
    }
    #[cfg(feature = "bytecode")]
    fn process_all_bytecode_with_debug(
        &mut self,
        id: &str,
        input: &str
    ) -> Result<String, AtpError> {
        let mut result = String::from(input);

        let dashes = 10;

        let tokens = self.transforms.get(id).ok_or_else(token_array_not_found(id))?;

        println!("PROCESSING STEP BY STEP:\n{}\n", "-".repeat(dashes));

        let mut context = GlobalExecutionContext::new();

        for (counter, token) in (0_i64..).zip(tokens.iter()) {
            let temp = parse_token(
                token.as_ref(),
                result.as_str(),
                &mut self.errors,
                &mut context
            )?;
            println!(
                "Step: [{}] => [{}]\nInstruction: {}\nBefore: {}\nAfter: {}\n",
                counter.to_string().blue(),
                (counter + 1).to_string().blue(),
                token.to_atp_line().yellow(),
                result.red(),
                temp.green()
            );

            if (counter as usize) < tokens.len() {
                println!("{}\n", "-".repeat(dashes));
            }

            result = temp;
        }

        Ok(result.to_string())
    }
    #[cfg(feature = "bytecode")]
    fn process_single_bytecode_with_debug(
        &mut self,
        token: Box<dyn TokenMethods>,
        input: &str
    ) -> Result<String, AtpError> {
        let output = match token.transform(input) {
            Ok(x) => x,
            Err(e) => {
                self.errors.add_error(e.clone());
                return Err(e);
            }
        };
        println!(
            "Step: [{}] => [{}]\nInstruction: {}\nBefore: {}\nAfter: {}\n",
            (0).to_string().blue(),
            (1).to_string().blue(),
            token.to_atp_line().yellow(),
            input.red(),
            output.green()
        );

        Ok(output)
    }
}
