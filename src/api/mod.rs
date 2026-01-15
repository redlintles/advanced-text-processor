pub mod atp_builder;
pub mod atp_processor;
pub mod conditional_builder;
pub mod block_builder;

use crate::api::block_builder::BlockBuilder;
use crate::api::conditional_builder::ConditionalBuilderEach;
use crate::globals::var::TokenWrapper;
use crate::tokens::instructions::cblk::Cblk;
use crate::tokens::instructions::ifdc;
use crate::tokens::transforms::ate::Ate;
use crate::tokens::transforms::tbs::Tbs;
use crate::tokens::transforms::tls::Tls;
use crate::tokens::transforms::trs::Trs;
use crate::tokens::{ transforms::*, InstructionMethods };
use crate::utils::errors::{ AtpError };
use crate::utils::params::AtpParamTypes;

pub trait AtpBuilderMethods: Sized {
    fn push_token(&mut self, t: impl Into<TokenWrapper>) -> Result<(), AtpError>;

    /// TBS - Trim Both Sides
    ///
    /// Removes whitespace characters from both the left and right sides of the input.
    ///
    /// See Also:
    ///
    /// - [`Tls` - Trim Left Side](crate::tokens::transforms::tls)
    /// - [`Trs` - Trim Right Side](crate::tokens::transforms::trs)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().trim_both_sides().build();
    /// let input = "   banana   ";
    ///
    /// assert_eq!(processor.process_all(&id, input), Ok("banana".to_string()));
    /// ```
    fn trim_both_sides(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(Tbs::default());
        self.push_token(tok)?;
        Ok(self)
    }

    /// TLS - Trim Left Side
    ///
    /// Removes whitespace characters exclusively from the left side of the input.
    ///
    /// See Also:
    ///
    /// - [`Trs` - Trim Right Side](crate::tokens::transforms::trs)
    /// - [`Tbs` - Trim Both Sides](crate::tokens::transforms::tbs)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().trim_left_side().build();
    /// let input = "   banana  ";
    ///
    /// assert_eq!(processor.process_all(&id, input), Ok("banana  ".to_string()));
    /// ```
    fn trim_left_side(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(Tls::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// TRS - Trim Right Side
    ///
    /// Removes whitespace characters exclusively from the right side of the input.
    ///
    /// See Also:
    ///
    /// - [`Tls` - Trim Left Side](crate::tokens::transforms::tls)
    /// - [`Tbs` - Trim Both Sides](crate::tokens::transforms::tbs)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().trim_right_side().build();
    /// let input = "  banana   ";
    ///
    /// assert_eq!(processor.process_all(&id, input), Ok("  banana".to_string()));
    /// ```
    fn trim_right_side(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(Trs::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// ATE - Add To End
    ///
    /// Appends the provided `text` to the end of the input string.
    ///
    /// See Also:
    ///
    /// - [`Atb` - Add To Beginning](crate::tokens::transforms::atb)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().add_to_end("!").build();
    /// let input = "banana";
    ///
    /// assert_eq!(processor.process_all(&id, input), Ok("banana!".to_string()));
    /// ```
    fn add_to_end(&mut self, text: &str) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(Ate::new(text));
        self.push_token(tok)?;
        Ok(self)
    }
    /// ATB - Add To Beginning
    ///
    /// Inserts the provided `text` at the beginning of the input string.
    ///
    /// See Also:
    ///
    /// - [`Ate` - Add To End](crate::tokens::transforms::ate)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().add_to_beginning("x").build();
    /// let input = "banana";
    ///
    /// assert_eq!(processor.process_all(&id, input), Ok("xbanana".to_string()));
    /// ```

    fn add_to_beginning(&mut self, text: &str) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(atb::Atb::new(text));
        self.push_token(tok)?;
        Ok(self)
    }
    /// DLF - Delete First
    ///
    /// Removes the first character of the input string.
    ///
    /// See Also:
    ///
    /// - [`Dll` - Delete Last](crate::tokens::transforms::dll)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().delete_first().build();
    /// let input = "banana";
    ///
    /// assert_eq!(processor.process_all(&id, input), Ok("anana".to_string()));
    /// ```

    fn delete_first(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(dlf::Dlf::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// DLL - Delete Last
    ///
    /// Removes the last character of the input string.
    ///
    /// See Also:
    ///
    /// - [`Dlf` - Delete First](crate::tokens::transforms::dlf)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().delete_last().build();
    /// let input = "banana";
    ///
    /// assert_eq!(processor.process_all(&id, input), Ok("banan".to_string()));
    /// ```

    fn delete_last(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(dll::Dll::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// DLA - Delete After
    ///
    /// Removes all characters after the provided `index`, keeping the content from `0..=index`.
    ///
    /// See Also:
    ///
    /// - [`Dlb` - Delete Before](crate::tokens::transforms::dlb)
    /// - [`Dlc` - Delete Chunk](crate::tokens::transforms::dlc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().delete_after(2).build();
    /// let input = "banana";
    ///
    /// assert_eq!(processor.process_all(&id, input), Ok("ban".to_string()));
    /// ```

    fn delete_after(&mut self, index: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(dla::Dla::new(index));
        self.push_token(tok)?;
        Ok(self)
    }
    /// DLB - Delete Before
    ///
    /// Removes all characters before the provided `index`, keeping the content from `index..`.
    ///
    /// See Also:
    ///
    /// - [`Dla` - Delete After](crate::tokens::transforms::dla)
    /// - [`Dlc` - Delete Chunk](crate::tokens::transforms::dlc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().delete_before(3).build();
    /// let input = "banana";
    ///
    /// assert_eq!(processor.process_all(&id, input), Ok("ana".to_string()));
    /// ```

    fn delete_before(&mut self, index: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(dlb::Dlb::new(index));
        self.push_token(tok)?;
        Ok(self)
    }
    /// DLC - Delete Chunk
    ///
    /// Removes all characters between `start_index` and `end_index` (inclusive),
    /// effectively slicing out that segment from the input string.
    ///
    /// See Also:
    ///
    /// - [`Dla` - Delete After](crate::tokens::transforms::dla)
    /// - [`Dlb` - Delete Before](crate::tokens::transforms::dlb)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().delete_chunk(1, 3).build();
    /// let input = "banana";
    ///
    /// assert_eq!(processor.process_all(&id, input), Ok("bna".to_string()));
    /// ```

    fn delete_chunk(
        &mut self,
        start_index: usize,
        end_index: usize
    ) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(dlc::Dlc::new(start_index, end_index)?);
        self.push_token(tok)?;
        Ok(self)
    }
    /// RAW - Replace All With
    ///
    /// Replaces **all** occurrences of `pattern` with `text_to_replace`.
    ///
    /// See Also:
    ///
    /// - [`Replace First`](crate::tokens::transforms::rfw)
    /// - [`Replace Last`](crate::tokens::transforms::rlw)
    /// - [`Replace Nth`](crate::tokens::transforms::rnw)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) =
    ///     AtpBuilder::new().replace_all_with("a", "x").build();
    ///
    /// let input = "banana";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id,&input),
    ///     Ok("bxnxnx".to_string())
    /// );
    /// ```

    fn replace_all_with(
        &mut self,
        pattern: &str,
        text_to_replace: &str
    ) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(match
            raw::Raw::new(pattern, text_to_replace)
        {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        });

        self.push_token(tok)?;
        Ok(self)
    }
    /// RFW - Replace First With
    ///
    /// Replaces only the **first** occurrence of `pattern` with `text_to_replace`.
    ///
    /// See Also:
    ///
    /// - [`Replace All`](crate::tokens::transforms::raw)
    /// - [`Replace Last`](crate::tokens::transforms::rlw)
    /// - [`Replace Nth`](crate::tokens::transforms::rnw)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) =
    ///     AtpBuilder::new().replace_first_with("a", "x").build();
    ///
    /// let input = "banana";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("bxnana".to_string())
    /// );
    /// ```

    fn replace_first_with(
        &mut self,
        pattern: &str,
        text_to_replace: &str
    ) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(match
            rfw::Rfw::new(pattern, text_to_replace)
        {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        });

        self.push_token(tok)?;
        Ok(self)
    }
    /// RLW - Replace Last With
    ///
    /// Replaces only the **last** occurrence of `pattern` with `text_to_replace`.
    ///
    /// See Also:
    ///
    /// - [`Replace First`](crate::tokens::transforms::rfw)
    /// - [`Replace All`](crate::tokens::transforms::raw)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) =
    ///     AtpBuilder::new().replace_last_with("a", "x").build();
    ///
    /// let input = "banana";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id,&input),
    ///     Ok("bananx".to_string())
    /// );
    /// ```

    fn replace_last_with(
        &mut self,
        pattern: &str,
        text_to_replace: &str
    ) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(match
            rlw::Rlw::new(pattern, text_to_replace)
        {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        });

        self.push_token(tok)?;
        Ok(self)
    }
    /// RNW - Replace Nth With
    ///
    /// Replaces the **nth** occurrence (0-based) of `pattern`
    /// with `text_to_replace`. If the index does not exist, no changes occur.
    ///
    /// See Also:
    ///
    /// - [`Replace All`](crate::tokens::transforms::raw)
    /// - [`Replace First`](crate::tokens::transforms::rfw)
    /// - [`Replace Last`](crate::tokens::transforms::rlw)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) =
    ///     AtpBuilder::new().replace_nth_with("a", "x", 1).build();
    ///
    /// let input = "banana";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("banxna".to_string())
    /// );
    /// ```

    fn replace_nth_with(
        &mut self,
        pattern: &str,
        text_to_replace: &str,
        index: usize
    ) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(match
            rnw::Rnw::new(pattern, text_to_replace, index)
        {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        });

        self.push_token(tok)?;
        Ok(self)
    }
    /// RCW - Replace Count With
    ///
    /// Replaces up to **count** occurrences of `pattern` with `text_to_replace`,
    /// scanning from left to right.
    ///
    /// See Also:
    ///
    /// - [`Replace All`](crate::tokens::transforms::raw)
    /// - [`Replace Nth`](crate::tokens::transforms::rnw)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) =
    ///     AtpBuilder::new().replace_count_with("a", "x", 2).build();
    ///
    /// let input = "banana";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("bxnxna".to_string())
    /// );
    /// ```

    fn replace_count_with(
        &mut self,
        pattern: &str,
        text_to_replace: &str,
        count: usize
    ) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(match
            rcw::Rcw::new(pattern, text_to_replace, count)
        {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        });

        self.push_token(tok)?;
        Ok(self)
    }
    /// RTL - Rotate Left
    ///
    /// Rotates the characters of the input to the **left** `times` positions.
    ///
    /// `"abcd".rotate_left(1)` → `"bcda"`
    ///
    /// See Also:
    ///
    /// - [`Rotate Right`](crate::tokens::transforms::rtr)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().rotate_left(2).build();
    ///
    /// let input = "abcd";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("cdab".to_string())
    /// );
    /// ```

    fn rotate_left(&mut self, times: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(rtl::Rtl::new(times));
        self.push_token(tok)?;
        Ok(self)
    }
    /// RTR - Rotate Right
    ///
    /// Rotates the characters of the input to the **right** `times` positions.
    ///
    /// `"abcd".rotate_right(1)` → `"dabc"`
    ///
    /// See Also:
    ///
    /// - [`Rotate Left`](crate::tokens::transforms::rtl)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().rotate_right(1).build();
    ///
    /// let input = "abcd";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("dabc".to_string())
    /// );
    /// ```

    fn rotate_right(&mut self, times: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(rtr::Rtr::new(times));
        self.push_token(tok)?;
        Ok(self)
    }
    /// RPT - Repeat
    ///
    /// Repeats the entire input string `times` times.
    ///
    /// See Also:
    ///
    /// - [`Pad Right`](crate::tokens::transforms::padr)
    /// - [`Pad Left`](crate::tokens::transforms::padl)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().repeat(3).build();
    ///
    /// let input = "hi";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("hihihi".to_string())
    /// );
    /// ```

    fn repeat(&mut self, times: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(rpt::Rpt::new(times));
        self.push_token(tok)?;
        Ok(self)
    }

    /// SLT - Select
    ///
    /// Extracts a substring from `start_index` to `end_index` (inclusive).
    ///
    /// See Also:
    ///
    /// - [`Delete Chunk`](crate::tokens::transforms::dlc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) =
    ///     AtpBuilder::new().select(1, 3).unwrap().build();
    ///
    /// let input = "abcdef";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("bcd".to_string())
    /// );
    /// ```

    fn select(&mut self, start_index: usize, end_index: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(slt::Slt::new(start_index, end_index)?);
        self.push_token(tok)?;
        Ok(self)
    }

    /// TUA - To Uppercase All
    ///
    /// Converts all characters of the input string to uppercase.
    ///
    /// See Also:
    ///
    /// - [`To Lowercase All`](crate::tokens::transforms::tla)
    /// - [`To Uppercase Chunk`](crate::tokens::transforms::tucc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().to_uppercase_all().build();
    ///
    /// let input = "banana";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("BANANA".to_string())
    /// );
    /// ```

    fn to_uppercase_all(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(tua::Tua::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// TLA - To Lowercase All
    ///
    /// Converts all characters of the input string to lowercase.
    ///
    /// See Also:
    ///
    /// - [`To Uppercase All`](crate::tokens::transforms::tua)
    /// - [`To Lowercase Chunk`](crate::tokens::transforms::tlcc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new().to_lowercase_all().build();
    ///
    /// let input = "BaNaNa";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("banana".to_string())
    /// );
    /// ```

    fn to_lowercase_all(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(tla::Tla::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// TUCS - To Uppercase Single
    ///
    /// Converts only the character at `index` to uppercase.
    /// If the index is out of range, no character is modified.
    ///
    /// See Also:
    ///
    /// - [`To Lowercase Single`](crate::tokens::transforms::tlcs)
    /// - [`To Uppercase Chunk`](crate::tokens::transforms::tucc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) =
    ///     AtpBuilder::new().to_uppercase_single(1).build();
    ///
    /// let input = "banana";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id,&input),
    ///     Ok("bAnana".to_string())
    /// );
    /// ```

    fn to_uppercase_single(&mut self, index: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(tucs::Tucs::new(index));
        self.push_token(tok)?;
        Ok(self)
    }
    /// TLCS - To Lowercase Single
    ///
    /// Converts only the character at `index` to lowercase.
    /// If the index is out of range, no character is modified.
    ///
    /// See Also:
    ///
    /// - [`To Uppercase Single`](crate::tokens::transforms::tucs)
    /// - [`To Lowercase Chunk`](crate::tokens::transforms::tlcc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) =
    ///     AtpBuilder::new().to_lowercase_single(0).build();
    ///
    /// let input = "Banana";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id,&input),
    ///     Ok("banana".to_string())
    /// );
    /// ```

    fn to_lowercase_single(&mut self, index: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(tlcs::Tlcs::new(index));
        self.push_token(tok)?;
        Ok(self)
    }
    /// TUCC - To Uppercase Chunk
    ///
    /// Converts a substring between `start_index` and `end_index` (inclusive)
    /// to uppercase.
    /// Returns an error if the indices are invalid.
    ///
    /// See Also:
    ///
    /// - [`To Lowercase Chunk`](crate::tokens::transforms::tlcc)
    /// - [`To Uppercase All`](crate::tokens::transforms::tua)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let builder = AtpBuilder::new()
    ///     .to_uppercase_chunk(1, 3)
    ///     .unwrap(); // required before build()
    ///
    /// let (mut processor, id) = builder.build();
    ///
    /// let input = "abcdef";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id,&input),
    ///     Ok("aBCDef".to_string())
    /// );
    /// ```

    fn to_uppercase_chunk(
        &mut self,
        start_index: usize,
        end_index: usize
    ) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(
            tucc::Tucc::new(start_index, end_index)?
        );
        self.push_token(tok)?;
        Ok(self)
    }
    /// TLCC - To Lowercase Chunk
    ///
    /// Converts a substring between `start_index` and `end_index` (inclusive)
    /// to lowercase.
    /// Returns an error if the indices are invalid.
    ///
    /// See Also:
    ///
    /// - [`To Uppercase Chunk`](crate::tokens::transforms::tucc)
    /// - [`To Lowercase All`](crate::tokens::transforms::tla)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let builder = AtpBuilder::new()
    ///     .to_lowercase_chunk(2, 4)
    ///     .unwrap();
    ///
    /// let (mut processor, id) = builder.build();
    ///
    /// let input = "ABCD EF";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id,&input),
    ///     Ok("ABcd ef".to_string())
    /// );
    /// ```

    fn to_lowercase_chunk(
        &mut self,
        start_index: usize,
        end_index: usize
    ) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(
            tlcc::Tlcc::new(start_index, end_index)?
        );
        self.push_token(tok)?;
        Ok(self)
    }

    /// CFW - Capitalize First Word
    ///
    /// Capitalizes the **first word** of the input string.
    /// A "word" is defined as the first contiguous sequence of non-whitespace characters.
    ///
    /// See Also:
    ///
    /// - [`Capitalize Last Word`](crate::tokens::transforms::clw) // expected token name
    /// - [`Capitalize Chunk`](crate::tokens::transforms::ctc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) =
    ///     AtpBuilder::new().capitalize_first_word().build();
    ///
    /// let input = "hello world";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id,&input),
    ///     Ok("Hello world".to_string())
    /// );
    /// ```

    fn capitalize_first_word(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(cfw::Cfw::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// CLW - Capitalize Last Word
    ///
    /// Capitalizes the **last word** of the input string.
    ///
    /// See Also:
    ///
    /// - [`Capitalize First Word`](crate::tokens::transforms::cfw)
    /// - [`Capitalize Chunk`](crate::tokens::transforms::ctc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) =
    ///     AtpBuilder::new().capitalize_last_word().build();
    ///
    /// let input = "hello world";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id,&input),
    ///     Ok("hello World".to_string())
    /// );
    /// ```

    fn capitalize_last_word(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(clw::Clw::default());
        self.push_token(tok)?;
        Ok(self)
    }

    /// SSLT - Split Select
    ///
    /// Splits the input string using `pattern` and selects the part at `index`.
    /// If the index does not exist, returns an empty string.
    ///
    /// See Also:
    ///
    /// - [`Split Remove`](crate::tokens::transforms::srmv)
    /// - [`Select`](crate::tokens::transforms::slt)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) =
    ///     AtpBuilder::new().split_select("-", 1).build();
    ///
    /// let input = "aa-bb-cc";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id,&input),
    ///     Ok("bb".to_string())
    /// );
    /// ```

    fn split_select(&mut self, pattern: &str, index: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(match sslt::Sslt::new(pattern, index) {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        });

        self.push_token(tok)?;
        Ok(self)
    }
    /// CTC - Capitalize Chunk
    ///
    /// Capitalizes the substring between `start_index` and `end_index` (inclusive).
    /// Returns an error if the indices are invalid.
    ///
    /// See Also:
    ///
    /// - [`Capitalize First Word`](crate::tokens::transforms::cfw)
    /// - [`Capitalize Last Word`](crate::tokens::transforms::clw)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let builder = AtpBuilder::new()
    ///     .capitalize_chunk(1, 3)
    ///     .unwrap();
    ///
    /// let (mut processor, id) = builder.build();
    ///
    /// let input = "abcdef";
    ///
    /// assert_eq!(
    ///     processor.process_all(&id,&input),
    ///     Ok("aBCDef".to_string())
    /// );
    /// ```

    fn capitalize_chunk(
        &mut self,
        start_index: usize,
        end_index: usize
    ) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(ctc::Ctc::new(start_index, end_index)?);
        self.push_token(tok)?;
        Ok(self)
    }
    /// CTR - Capitalize Range
    ///
    /// Capitalizes all characters in `input` from `start_index` (inclusive) to `end_index`
    /// (exclusive).
    /// If the indices are invalid, an `AtpError` is returned at build-time.
    ///
    /// See Also:
    ///
    /// - [`Ctc` - Capitalize Chunk](crate::tokens::transforms::ctc)
    /// - [`Cts` - Capitalize Single Word](crate::tokens::transforms::cts)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let builder = AtpBuilder::new()
    ///     .capitalize_range(1, 4)
    ///     .unwrap(); // required because this method returns Result
    ///
    /// let (mut processor, id) = builder.build();
    ///
    /// let input = "abcdef";
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("aBCDef".to_string())
    /// );
    /// ```
    fn capitalize_range(
        &mut self,
        start_index: usize,
        end_index: usize
    ) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(ctr::Ctr::new(start_index, end_index)?);
        self.push_token(tok)?;
        Ok(self)
    }
    /// CTS - Capitalize Single Word
    ///
    /// Capitalizes the word located at the given `index` in `input`.
    /// Words are delimited according to Unicode whitespace rules.
    ///
    /// See Also:
    ///
    /// - [`Cfw` - Capitalize First Word](crate::tokens::transforms::cfw)
    /// - [`Ctc` - Capitalize Chunk](crate::tokens::transforms::ctc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new()
    ///     .capitalize_single_word(2)
    ///     .build();
    ///
    /// let input = "hello brave world";
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("hello brave World".to_string())
    /// );
    /// ```
    fn capitalize_single_word(&mut self, index: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(cts::Cts::new(index));
        self.push_token(tok)?;
        Ok(self)
    }
    /// URLE - URL Encode
    ///
    /// Converts the entire `input` string into its URL-encoded form
    /// according to RFC 3986 percent-encoding rules.
    ///
    /// See Also:
    ///
    /// - [`Urld` - URL Decode](crate::tokens::transforms::urld)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new()
    ///     .to_url_encoded()
    ///     .build();
    ///
    /// let input = "hello world!";
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("hello%20world%21".to_string())
    /// );
    /// ```

    fn to_url_encoded(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(urle::Urle::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// URLD - URL Decode
    ///
    /// Decodes a URL-encoded string into its normal representation.
    /// Invalid percent-encoded sequences remain unchanged.
    ///
    /// See Also:
    ///
    /// - [`Urle` - URL Encode](crate::tokens::transforms::urle)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new()
    ///     .to_url_decoded()
    ///     .build();
    ///
    /// let input = "hello%20world%21";
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("hello world!".to_string())
    /// );
    /// ```

    fn to_url_decoded(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(urld::Urld::default());
        self.push_token(tok)?;
        Ok(self)
    }

    /// REV - Reverse Text
    ///
    /// Reverses all characters in the input string.
    ///
    /// This operation is Unicode-aware and preserves grapheme clusters.
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new()
    ///     .to_reverse()
    ///     .build();
    ///
    /// let input = "abc";
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("cba".to_string())
    /// );
    /// ```
    fn to_reverse(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(rev::Rev::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// SPLC - Split Characters
    ///
    /// Splits the entire input string into individual characters separated by spaces.
    /// Grapheme clusters are preserved (Unicode-aware).
    ///
    /// Example: `"abc"` → `"a b c"`
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new()
    ///     .split_characters()
    ///     .build();
    ///
    /// let input = "hello";
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("h e l l o".to_string())
    /// );
    /// ```

    fn split_characters(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(splc::Splc::default());
        self.push_token(tok)?;
        Ok(self)
    }

    /// HTMLE - HTML Escape
    ///
    /// Escapes HTML special characters such as `<`, `>`, `"`, `'`, `&`.
    /// Useful for preventing HTML injection or rendering raw text.
    ///
    /// See Also:
    ///
    /// - [`Htmlu` - HTML Unescape](crate::tokens::transforms::htmlu)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new()
    ///     .to_html_escaped()
    ///     .build();
    ///
    /// let input = "<b>Hello</b>";
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("&lt;b&gt;Hello&lt;/b&gt;".to_string())
    /// );
    /// ```

    fn to_html_escaped(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(htmle::Htmle::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// HTMLU - HTML Unescape
    ///
    /// Converts HTML escaped entities back into their literal characters.
    /// Example: `"&lt;" → "<"`
    ///
    /// See Also:
    ///
    /// - [`Htmle` - HTML Escape](crate::tokens::transforms::htmle)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::AtpBuilder;
    /// use atp::builder::atp_processor::AtpProcessorMethods;
    ///
    /// let (mut processor, id) = AtpBuilder::new()
    ///     .to_html_unescaped()
    ///     .build();
    ///
    /// let input = "&lt;b&gt;Hi&lt;/b&gt;";
    /// assert_eq!(
    ///     processor.process_all(&id, &input),
    ///     Ok("<b>Hi</b>".to_string())
    /// );
    /// ```
    fn to_html_unescaped(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(htmlu::Htmlu::default());
        self.push_token(tok)?;
        Ok(self)
    }

    /// To Json Escaped
    ///
    /// Escapes JSON characters of `string``
    ///
    /// See Also:
    ///
    /// - [JSONU - To json unescaped](crate::tokens::transforms::jsonu)
    ///
    /// # Example:
    ///
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().to_json_escaped().build();
    /// let input = "{banana: '10'}";
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("\"{banana: '10'}\"".to_string()));
    /// ```

    fn to_json_escaped(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(jsone::Jsone::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// To Json Unescaped
    ///
    /// Unescapes JSON characters of `string``
    ///
    /// See Also:
    ///
    /// - [JSONE - To json escaped](crate::tokens::transforms::jsone)
    ///
    /// # Example:
    ///
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().to_json_unescaped().build();
    /// let input = "\"{banana: '10'}\"";
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("{banana: '10'}".to_string()));
    /// ```
    fn to_json_unescaped(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(jsonu::Jsonu::default());
        self.push_token(tok)?;
        Ok(self)
    }

    /// Insert
    ///
    /// Inserts `text` after `index` of `string`
    ///
    /// See Also:
    ///
    /// - [ATB - Add to Beginning](crate::tokens::transforms::atb)
    /// - [ATE - Add to End](crate::tokens::transforms::ate)
    ///
    /// # Example:
    ///
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().insert(1, " laranja").build();
    /// let input = "banana";
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("ba laranjanana".to_string()));
    /// ```
    fn insert(&mut self, index: usize, text_to_insert: &str) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(ins::Ins::new(index, text_to_insert));
        self.push_token(tok)?;
        Ok(self)
    }

    /// To Lowercase Word
    ///
    /// Lowercases a single word of `string`
    ///
    /// See Also:
    ///
    /// - [TUCW - To Uppercase Word](crate::tokens::transforms::tucw)
    ///
    /// # Example:
    ///
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().to_lowercase_word(1).build();
    /// let input = "BANANA LARANJA CHEIA DE CANJA";
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("BANANA laranja CHEIA DE CANJA".to_string()));
    /// ```
    fn to_lowercase_word(&mut self, index: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(tlcw::Tlcw::new(index));
        self.push_token(tok)?;
        Ok(self)
    }
    /// To Uppercase Word
    ///
    /// Uppercases a single word of `string`
    ///
    /// See Also:
    ///
    /// - [TLCW - To Lowercase Word](crate::tokens::transforms::tlcw)
    ///
    /// # Example:
    ///
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().to_uppercase_word(1).build();
    /// let input = "banana laranja cheia de canja";
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("banana LARANJA cheia de canja".to_string()));
    /// ```
    fn to_uppercase_word(&mut self, index: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(tucw::Tucw::new(index));
        self.push_token(tok)?;
        Ok(self)
    }

    /// Join to kebab-case
    ///
    /// If `input` is a string whose words are separated by spaces, join `input` as a lowercased kebab-case string
    ///
    /// See Also:
    ///
    /// - [`Jpsc` - Join to Pascal Case](crate::tokens::transforms::jpsc)
    /// - [`Jsnc` - Join to Snake Case](crate::tokens::transforms::jsnc)
    /// - [`Jcmc` - Join to Camel Case](crate::tokens::transforms::jcmc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().join_to_kebab_case().build();
    /// let input = "banana laranja cheia de canja";
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("banana-laranja-cheia-de-canja".to_string()));
    ///

    fn join_to_kebab_case(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(jkbc::Jkbc::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// Join to snake_case
    ///
    /// If `input` is a string whose words are separated by spaces, join `input` as a lowercased snake_case string
    ///
    /// See Also:
    ///
    /// - [`Jpsc` - Join to Pascal Case](crate::tokens::transforms::jpsc)
    /// - [`Jkbc` - Join to Kebab Case](crate::tokens::transforms::jkbc)
    /// - [`Jcmc` - Join to Camel Case](crate::tokens::transforms::jcmc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().join_to_snake_case().build();
    /// let input = "banana laranja cheia de canja";
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("banana_laranja_cheia_de_canja".to_string()));
    ///
    fn join_to_snake_case(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(jsnc::Jsnc::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// Join to camelCase
    ///
    /// If `input` is a string whose words are separated by spaces, join `input` as a camelCase string
    ///
    /// See Also:
    ///
    /// - [`Jpsc` - Join to Pascal Case](crate::tokens::transforms::jpsc)
    /// - [`Jsnc` - Join to Snake Case](crate::tokens::transforms::jsnc)
    /// - [`Jkbc` - Join to Kebab Case](crate::tokens::transforms::jkbc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().join_to_camel_case().build();
    /// let input = "banana laranja cheia de canja";
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("bananaLaranjaCheiaDeCanja".to_string()));
    /// ```
    fn join_to_camel_case(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(jcmc::Jcmc::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// Join to PascalCase
    ///
    /// If `input` is a string whose words are separated by spaces, join `input` as a camelCase string
    ///
    /// See Also:
    ///
    /// - [`Jsnc` - Join to Snake Case](crate::tokens::transforms::jsnc)
    /// - [`Jcmc` - Join to Camel Case](crate::tokens::transforms::jcmc)
    /// - [`Jkbc` - Join to Kebab Case](crate::tokens::transforms::jkbc)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().join_to_pascal_case().build();
    /// let input = "banana laranja cheia de canja";
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("BananaLaranjaCheiaDeCanja".to_string()));
    /// ```
    fn join_to_pascal_case(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(jpsc::Jpsc::default());
        self.push_token(tok)?;
        Ok(self)
    }
    /// PADL - Pad Left
    ///
    /// Repeats `text` characters until `max_len` is reached, and then insert the result at the start of `input`
    ///
    /// See Also:
    ///
    /// - [`Padr` - Pad Left](crate::tokens::transforms::padr)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().pad_left("x", 7).build();
    /// let input = "banana";
    ///
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("xbanana".to_string()));
    /// ```
    fn pad_left(&mut self, text: &str, times: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(padl::Padl::new(text, times));
        self.push_token(tok)?;
        Ok(self)
    }
    /// PADR - Pad Right
    ///
    /// Repeats `text` characters until `max_len` is reached, and then insert the result at the end of `input`
    ///
    /// See Also:
    ///
    /// - [`Padl` - Pad Left](crate::tokens::transforms::padl)
    ///
    /// # Example:
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().pad_right("x", 7).build();
    /// let input = "banana";
    ///
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("bananax".to_string()));
    /// ```
    fn pad_right(&mut self, text: &str, times: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(padr::Padr::new(text, times));
        self.push_token(tok)?;
        Ok(self)
    }
    /// RMWS - Remove Whitespace
    ///
    /// Removes all whitespaces in `input`
    ///
    /// # Example:
    ///
    /// /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().remove_whitespace().build();
    /// let input = "banana laranja cheia de canja";
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("bananalaranjacheiadecanja".to_string()));
    /// ```
    fn remove_whitespace(&mut self) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(rmws::Rmws::default());
        self.push_token(tok)?;
        Ok(self)
    }

    /// DLS - Delete Single
    ///
    /// Delete's a single character specified by `index` in `input`
    ///
    /// It will throw an `AtpError` if index does not exists in `input`
    ///
    /// # Example:
    ///
    /// ```rust
    /// use atp::builder::atp_builder::{AtpBuilder};
    /// use atp::builder::atp_processor::{AtpProcessorMethods};
    ///
    /// let (mut processor, id) = AtpBuilder::new().delete_single(3).build();
    /// let input = "banana";
    ///
    /// assert_eq!(processor.process_all(&id,&input), Ok("banna".to_string()));
    /// ```
    fn delete_single(&mut self, index: usize) -> Result<&mut Self, AtpError> {
        let tok: Box<dyn InstructionMethods> = Box::new(dls::Dls::new(index));
        self.push_token(tok)?;
        Ok(self)
    }
}

pub trait AtpConditionalMethods: AtpBuilderMethods {
    fn if_do_contains_each<F>(&mut self, value: &str, f: F) -> Result<&mut Self, AtpError>
        where F: FnOnce(&mut ConditionalBuilderEach) -> Result<(), AtpError>
    {
        let params = vec![AtpParamTypes::String(value.to_string())];
        let token: Box<dyn InstructionMethods> = Box::new(ifdc::Ifdc::default());
        let mut conditional_builder = ConditionalBuilderEach::new(token, params);

        f(&mut conditional_builder)?;

        let result = conditional_builder.build();

        for token in result.into_iter() {
            self.push_token(token)?;
        }

        Ok(self)
    }
}

pub trait AtpBlockMethods: AtpBuilderMethods {
    fn block_assoc<F>(&mut self, block_name: &'static str, f: F) -> Result<&mut Self, AtpError>
        where F: FnOnce(&mut BlockBuilder) -> Result<(), AtpError>
    {
        let mut block_builder = BlockBuilder::new(block_name);

        f(&mut block_builder)?;

        let result = block_builder.build();

        for token in result.into_iter() {
            self.push_token(token)?;
        }
        Ok(self)
    }

    fn call_block(&mut self, block_name: &'static str) -> Result<&mut Self, AtpError> {
        let mut t: Box<dyn InstructionMethods> = Box::new(Cblk::default());

        t.from_params(&vec![AtpParamTypes::String(block_name.to_string())])?;

        self.push_token(t)?;
        Ok(self)
    }
}
