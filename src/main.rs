use std::{ fs::OpenOptions, io::{ self, Error, Read, Write }, path::PathBuf };
use atp::{
    builder::atp_processor::{ AtpProcessor, AtpProcessorMethods },
    utils::{
        cli::{ process_input_by_chunks, process_input_line_by_line, process_input_single_chunk },
        errors::AtpError,
    },
};
use clap::{ value_parser, Arg, ArgAction, Command };

#[derive(Clone, Copy, PartialEq, Debug)]
enum ReadMode {
    All,
    Line,
    Chunk(usize),
}

fn build_cli() -> Command {
    Command::new("atp")
        .version("0.1")
        .about("CLI for ATP(Advanced Text Processor)")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .required(true)
                .value_name("FILE")
                .value_parser(value_parser!(PathBuf))
                .help("Arquivo .atp ou .atpbc")
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(false)
                .value_name("INPUT")
                .value_parser(value_parser!(PathBuf))
                .help(
                    "The file that will be processed, if not specified, will read from stdin, otherwise, will use an empty string"
                )
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(false)
                .value_name("OUTPUT")
                .value_parser(value_parser!(PathBuf))
                .help(
                    "The file where the output will be stored, if not specified, the content will just be printed in stdout"
                )
        )
        .arg(
            Arg::new("read_mode")
                .short('r')
                .long("read-mode")
                .default_value("all")
                .required(false)
                .value_name("READ_MODE")
                .value_parser(
                    |s: &str| -> Result<ReadMode, Error> {
                        if s == "all" {
                            Ok(ReadMode::All)
                        } else if s == "line" {
                            Ok(ReadMode::Line)
                        } else if let Some(num) = s.strip_prefix("chunk-") {
                            num.parse::<usize>()
                                .map(ReadMode::Chunk)
                                .map_err(|_|
                                    Error::new(io::ErrorKind::InvalidInput, "Chunk size inválido")
                                )
                        } else {
                            Err(Error::new(io::ErrorKind::InvalidInput, "Modo inválido"))
                        }
                    }
                )
                .help(
                    "Input Read mode, default value is 'all', meaning it will read all file contents as a single string, other possible values are 'line', to read the file line by line, and 'chunk-X', meaning it will read the file in chunks of X characters"
                )
        )
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .required(false)
                .value_name("mode")
                .value_parser(["b", "t"])
                .default_value("t")
                .help(
                    "The ATP mode that will be used, default is 't', for text mode, you can also use 'b' for bytecode mode"
                )
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .required(false)
                .value_name("debug")
                .action(ArgAction::SetTrue)
                .help("Determines whether ATP will run in debug mode or not, default is false")
        )
}

fn process_by_mode(
    read_mode: &ReadMode,
    id: &str,
    data: &str,
    debug: bool,
    processor: &mut AtpProcessor
) -> Result<String, AtpError> {
    match read_mode {
        ReadMode::All => process_input_single_chunk(processor, id, data, debug),
        ReadMode::Line => process_input_line_by_line(processor, id, data, debug),
        ReadMode::Chunk(s) => process_input_by_chunks(processor, id, data, *s, debug),
    }
}

fn main() -> Result<(), AtpError> {
    let matches = build_cli().get_matches();

    let file = matches.get_one::<PathBuf>("file").unwrap();
    let input = matches.get_one::<PathBuf>("input");
    let output = matches.get_one::<PathBuf>("output");
    let atp_mode = matches.get_one::<String>("mode").unwrap();
    let read_mode = matches.get_one::<ReadMode>("read_mode").unwrap();
    let debug = matches.get_one::<bool>("debug").unwrap();

    if atp_mode == &"b" && file.extension().expect("Could not get input extension") != "atpbc" {
        panic!("You're using mode 'b'(bytecode), so the atp file must have the .atpbc extension!");
    }

    if !file.exists() {
        panic!("ATP file does not exists!");
    }

    let data: String = match input {
        Some(path) => {
            let mut b = String::new();

            if !file.exists() {
                panic!("The specified file does not exists");
            }

            let mut file = OpenOptions::new()
                .read(true)
                .open(path)
                .expect(&format!("Error opening file {}", path.display().to_string()));

            file.read_to_string(&mut b).expect("Error reading input file");

            b
        }
        None => {
            let mut b = String::new();

            io::stdin().read_to_string(&mut b).expect("Error while reading from stdin");

            b
        }
    };

    let mut result: String = String::new();

    if atp_mode == &"b" {
        let mut processor = AtpProcessor::new();
        let id = processor.read_from_bytecode_file(file)?;

        result = process_by_mode(read_mode, &id, &data, *debug, &mut processor)?;
    } else if atp_mode == &"t" {
        let mut processor = AtpProcessor::new();
        let id = processor.read_from_text_file(file)?;

        result = process_by_mode(read_mode, &id, &data, *debug, &mut processor)?;
    }

    match output {
        Some(p) => {
            if p.exists() {
                let mut f = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(p)
                    .expect("It was not possible to open the file for writing");

                f.write_all(result.as_bytes()).expect("Failed to write result to file");
            } else {
                let mut f = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(p)
                    .expect("It was not possible to open the file for writing");

                f.write_all(result.as_bytes()).expect("Failed to write result to file");
            }
        }
        None => {
            println!("Resultado do processamento: {}", result);
        }
    }

    Ok(())
}

#[cfg(feature = "test_access")]
#[cfg(test)]
mod atp_tests {
    mod parser_tests {
        use std::{ path::PathBuf, str::FromStr };
        use crate::{ build_cli, ReadMode };

        #[test]
        fn test_all_with_long_params() {
            let parser = build_cli();
            let c =
                "atp --file ./instructions.atpbc --input ./example.txt --output output.txt --debug --mode b --read-mode line";

            let arg_vec = shell_words::split(c).unwrap();

            let m = parser.try_get_matches_from(arg_vec).unwrap();

            let file = m.get_one::<PathBuf>("file").unwrap();
            let input = m.get_one::<PathBuf>("input").unwrap();
            let output = m.get_one::<PathBuf>("output").unwrap();
            let atp_mode = m.get_one::<String>("mode").unwrap();
            let read_mode = m.get_one::<ReadMode>("read_mode").unwrap();
            let debug = m.get_one::<bool>("debug").unwrap();

            assert_eq!(*file, PathBuf::from_str("./instructions.atpbc").unwrap());
            assert_eq!(*input, PathBuf::from_str("./example.txt").unwrap());
            assert_eq!(*output, PathBuf::from_str("output.txt").unwrap());
            assert_eq!(*atp_mode, "b".to_string());
            assert_eq!(*read_mode, ReadMode::Line);
            assert_eq!(*debug, true);
        }
        #[test]
        fn test_all_with_short_params() {
            let parser = build_cli();
            let c = "atp -f ./instructions.atpbc -i ./example.txt -o output.txt -d -m b -r line";

            let arg_vec = shell_words::split(c).unwrap();

            let m = parser.try_get_matches_from(arg_vec).unwrap();

            let file = m.get_one::<PathBuf>("file").unwrap();
            let input = m.get_one::<PathBuf>("input").unwrap();
            let output = m.get_one::<PathBuf>("output").unwrap();
            let atp_mode = m.get_one::<String>("mode").unwrap();
            let read_mode = m.get_one::<ReadMode>("read_mode").unwrap();
            let debug = m.get_one::<bool>("debug").unwrap();

            assert_eq!(*file, PathBuf::from_str("./instructions.atpbc").unwrap());
            assert_eq!(*input, PathBuf::from_str("./example.txt").unwrap());
            assert_eq!(*output, PathBuf::from_str("output.txt").unwrap());
            assert_eq!(*atp_mode, "b".to_string());
            assert_eq!(*read_mode, ReadMode::Line);
            assert_eq!(*debug, true);
        }
    }
}
