use std::{ fs::OpenOptions, io::{ self, Error, Read }, path::PathBuf };
use clap::{ value_parser, Arg, Command };

#[derive(Clone, Copy)]
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
                .required(true)
                .value_name("mode")
                .value_parser(["b", "t"])
                .default_value("t")
                .help(
                    "The ATP mode that will be used, default is 't', for text mode, you can also use 'b' for bytecode mode"
                )
        )
}

fn main() {
    let matches = build_cli().get_matches();

    let file = matches.get_one::<PathBuf>("file").unwrap();
    let input = matches.get_one::<Option<PathBuf>>("input").unwrap();
    let output = matches.get_one::<Option<PathBuf>>("output").unwrap();
    let atp_mode = matches.get_one::<char>("mode").unwrap();
    let read_mode = matches.get_one::<ReadMode>("read_mode").unwrap();

    println!("Arquivo Fonte: {}, Existe? {}", file.display().to_string(), file.exists());

    if atp_mode == &'b' && file.extension().expect("Could not get input extension") != "atpbc" {
        panic!("You're using mode 'b'(bytecode), so the atp file must have the .atpbc extension!");
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
}
