use carrot_utils::logger;
use clap::Parser;
use log::info;
use log::LevelFilter;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, help = "The path to input text.")]
    input: String,

    #[clap(
        short,
        long,
        default_value = "output.md",
        help = "The path to output text. Default value is output.md"
    )]
    output: String,

    #[clap(
        long,
        default_value = "Info",
        help = "Log level for command. Choose from Error, Warn, Info, Debug, Trace. Default value is Info."
    )]
    log_level: String,

    #[clap(
        long,
        help = "Convert the second and subsequent lines to plain text if this option uses."
    )]
    is_plane_text: bool,
}

#[derive(Debug, Deserialize)]
struct Config {
    is_plane_text: bool,
}

fn parse_body(line_text: String, is_blank_line: bool, config: &Config) -> String {
    if config.is_plane_text == true {
        if is_blank_line == true {
            format!("\n- {}\n", line_text)
        } else {
            format!("{}", line_text)
        }
    } else {
        if is_blank_line == true {
            format!("- {}", line_text)
        } else {
            format!("  - {}", line_text)
        }
    }
}

fn txt_to_md(input_path: &str, output_path: &str, config: &Config) {
    let mut output_file = File::create(output_path).unwrap();

    let input = File::open(input_path).unwrap();
    let buffer = BufReader::new(input);

    info!("Convert from {:?} to {:?}", input_path, output_path);
    let mut is_blank_line: bool = false;
    let mut subtitle = String::new();

    for line in buffer.lines() {
        let line_text: String = line.unwrap();
        if line_text == "" {
            is_blank_line = true;
        } else {
            if line_text.contains("#") {
                subtitle = format!("{}\n{}", subtitle, line_text);
            } else {
                if subtitle != "" {
                    write!(output_file, "{}\n\n", subtitle).unwrap();
                    subtitle = String::new();
                }
                let output_text = parse_body(line_text, is_blank_line, config);
                writeln!(output_file, "{}", output_text).unwrap();
            }
            is_blank_line = false;
        }
    }
}

fn main() {
    // arg
    let args = Args::parse();

    // set logger
    let log_level: LevelFilter = logger::get_log_level(&args.log_level);
    let _ = logger::Logger::new(
        "./data/{TIME_SEC}",
        "log_{TIME_SEC}.txt",
        log_level,
        log_level,
        LevelFilter::Debug,
    );

    let config = Config {
        is_plane_text: args.is_plane_text,
    };
    txt_to_md(&args.input, &args.output, &config);
}
