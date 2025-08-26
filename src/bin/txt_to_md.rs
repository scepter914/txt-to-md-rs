use clap::Parser;
use env_logger::{Builder, Env};
use log::{error, info};
use std::fs;
use std::io::{self};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(index = 1, default_value = "", help = "Input text.")]
    text: String,

    #[clap(short, long, help = "The path to input text.")]
    input: Option<PathBuf>,

    #[clap(
        short,
        long,
        default_value = "output.md",
        help = "The path to output text. Default value is output.md"
    )]
    output: PathBuf,

    #[clap(
        long,
        help = "Convert the second and subsequent lines to plain text if this option uses."
    )]
    is_plane_text: bool,
}

fn last_non_empty_is_heading(out: &[String]) -> bool {
    out.iter()
        .rev()
        .find(|s| !s.trim().is_empty())
        .map(|s| s.trim_start().starts_with('#'))
        .unwrap_or(false)
}

fn is_markdown_line(s: &str) -> bool {
    let t = s.trim_start();
    // Decide for markdown
    if t.starts_with('#')
        || t.starts_with("```")
        || t.starts_with("> ")
        || t.starts_with("- ")
        || t.starts_with("+ ")
        || t.starts_with("* ")
        || t.starts_with("-[")
        || t.starts_with("- [")
    {
        return true;
    }

    // Example: "1. foo" / "12) bar"
    let mut chars = t.chars().peekable();
    let mut saw_digit = false;
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            saw_digit = true;
            chars.next();
        } else {
            break;
        }
    }
    if saw_digit {
        if let Some(&c) = chars.peek() {
            if c == '.' || c == ')' {
                // If next is space or end, then regard as number lists
                let mut it = chars.clone();
                it.next(); // consume '.' or ')'
                if it.next().map_or(true, |x| x == ' ') {
                    return true;
                }
            }
        }
    }

    false
}

fn convert_text_to_md(input: &str, is_plane_text: bool) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut buf: Vec<String> = Vec::new();
    let mut in_code = false;

    let flush_buf = |out: &mut Vec<String>, buf: &mut Vec<String>| {
        if buf.is_empty() {
            return;
        }
        // Only convert the first line into a list item
        // The handling of the subsequent lines depends on the mode.
        let first = buf[0].trim();
        if !first.is_empty() {
            // If the immediately preceding non-empty line is a heading, insert a blank line before the list
            if last_non_empty_is_heading(out) {
                out.push(String::new());
            }
            out.push(format!("- {}", first));
        }
        if is_plane_text {
            out.push(String::new()); // Push empty line
            for line in buf.iter().skip(1) {
                out.push(line.to_string());
            }
        } else {
            for line in buf.iter().skip(1) {
                let t = line.trim();
                if !t.is_empty() {
                    out.push(format!("  - {}", t));
                }
            }
        }
        buf.clear();
    };

    for raw_line in input.lines() {
        let line = raw_line.to_string();
        let trimmed = line.trim_start();

        // code block
        if trimmed.starts_with("```") {
            flush_buf(&mut out, &mut buf);
            in_code = !in_code;
            out.push(line);
            continue;
        }

        if in_code {
            out.push(line);
            continue;
        }

        // empty line
        if trimmed.is_empty() {
            flush_buf(&mut out, &mut buf);
            continue;
        }

        // Use existed markdown line
        if is_markdown_line(trimmed) {
            flush_buf(&mut out, &mut buf);
            out.push(line);
            continue;
        }

        // Plain lines go into the paragraph buffer.
        buf.push(line);
    }

    // Flash last paragraph
    flush_buf(&mut out, &mut buf);

    // Lightly clean up to avoid extra consecutive blank lines at the end
    while out.last().map_or(false, |s| s.is_empty()) {
        out.pop();
    }

    out.join("\n")
}

fn convert_txt_file_to_md_file(input_path: PathBuf, output_path: PathBuf, is_plane_text: bool) {
    let input_path_disp = input_path.display();
    let output_path_disp = output_path.display();

    match fs::read_to_string(&input_path) {
        Ok(content) => {
            let converted = convert_text_to_md(&content, is_plane_text);
            if let Err(e) = fs::write(&output_path, converted) {
                error!("failed to write {}: {}", output_path_disp, e);
            } else {
                info!("wrote {}", output_path_disp);
            }
        }
        Err(e) => {
            error!("failed to read {}: {}", input_path_disp, e);
        }
    }
}

fn main() -> io::Result<()> {
    // Logger
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    // Arg
    let args = Args::parse();

    if args.input.is_some() {
        // Use a file as input
        convert_txt_file_to_md_file(args.input.unwrap(), args.output, args.is_plane_text);
    } else {
        // Use CLI as input
        let converted = convert_text_to_md(&args.text, args.is_plane_text);
        println!("{converted}");
    }

    Ok(())
}
