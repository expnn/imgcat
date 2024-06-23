use std::fs::{self, File};
use std::{env, io};
use std::io::Read;
use anyhow;
use anyhow::Context;
use base64::Engine;
use base64::prelude::{BASE64_STANDARD, BASE64_URL_SAFE};
// noinspection RsUnusedImport
use clap::{Parser, Args, ArgAction, arg};
use reqwest;

/// Display images inline in terminals support iTerm2's Inline Images Protocol
///
/// If you don't specify width or height an appropriate value will be chosen automatically.
/// The width and height are given as word 'auto' or number N followed by a unit:
///
///     N      character cells
///     Npx    pixels
///     N%     percent of the session's width or height
///     auto   the image's inherent size will be used to determine an appropriate dimension
///
/// If a type is provided, it is used as a hint to disambiguate."
/// The file type can be a mime type like text/markdown, a language name like Java, or a file extension like .c"
/// The file type can usually be inferred from the extension or its contents. -t is most useful when"
/// a filename is not available, such as whe input comes from a pipe."
///
/// Examples:
///
/// $ imgcat -W 250px -H 250px -s avatar.png
/// $ cat graph.png | imgcat -W 100%
/// $ imgcat -p -W 500px -u http://host.tld/path/to/image.jpg -W 80 -f image.png
/// $ cat url_list.txt | xargs imgcat -p -W 40 -u
/// $ imgcat -t application/json config.json
#[derive(Parser, Debug)]
#[command(version, about, long_about, verbatim_doc_comment)]
struct Cli {
    #[command(flatten)]
    input: Input,

    #[arg(short='t', long)]
    file_type: Option<String>,

    /// output width of the image
    #[arg(short='W', long)]
    width: Option<String>,

    /// output height of the image
    #[arg(short='H', long)]
    height: Option<String>,

    /// preserve aspect ratio when draw the image
    #[arg(short='s', long="stretch", action=ArgAction::SetFalse, default_value_t = true)]
    preserve_aspect_ratio: bool,

    /// whether to print the filename of the image or not
    #[arg(short, long)]
    print_filename: bool,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct Input {
    // /// read input from Stdin
    // #[arg(long)]
    // stdin: bool,

    /// read input image from URL
    #[arg(short, long)]
    url: Option<String>,

    /// read input image from local file
    #[arg(short, long)]
    file: Option<String>,
}

fn print_osc() {
    if let Ok(term) = env::var("TERM") {
        if term.starts_with("screen") || term.starts_with("tmux") {
            print!("\x1bPtmux;\x1b\x1b]");
        } else {
            print!("\x1b]");
        }
    } else {
        print!("\x1b]");
    }
}

fn print_image(
    image: &[u8],
    filename: Option<&str>,
    width: &Option<String>,
    height: &Option<String>,
    preserve_aspect_ratio: bool,
    file_type: &Option<String>,
) {
    print_osc();
    print!("1337;File=inline=1;size={}", image.len());

    if let Some(name) = filename {
        print!(";name={}", BASE64_URL_SAFE.encode(name));
    }

    if let Some(w) = width {
        print!(";width={w}");
    }

    if let Some(h) = height {
        print!(";height={h}");
    }

    print!(";preserveAspectRatio={}", preserve_aspect_ratio as u8);

    if let Some(ft) = file_type {
        print!(";type={ft}");
    }
    print!(":{}", BASE64_STANDARD.encode(image));
    print_st();

    println!();
    if let Some(name) = filename {
        println!("{name}");
    }
}

fn print_st() {
    if let Ok(term) = env::var("TERM") {
        if term.starts_with("screen") || term.starts_with("tmux") {
            print!("\x07\x1b\\");
        } else {
            print!("\x07");
        }
    } else {
        print!("\x07");
    }
}

fn read_image(input: &Input) -> anyhow::Result<Vec<u8>> {
    Ok(if let Some(f) = &input.file {
        let mut file = File::open(f)
            .with_context(|| format!("failed to open file {f}"))?;
        let metadata = fs::metadata(&f);
        let mut buffer = match metadata {
            Ok(m) => {vec![0; m.len() as usize]}
            Err(_) => {Vec::new()}
        };
        file.read(&mut buffer)
            .with_context(|| format!("failed to read from file {f}"))?;
        buffer
    } else if let Some(url) = &input.url {
        reqwest::blocking::get(url)
            .with_context(|| format!("failed to connect to {url}"))?
            .bytes()
            .with_context(|| format!("failed to fetch image data from {url}"))?
            .iter()
            .cloned()
            .collect()
    } else {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)
            .with_context(|| "failed to read stdin")?;
        buffer
    })
}

fn get_filename(input: &Input) -> Option<&str> {
    if let Some(f) = &input.file {
        Some(f.as_str())
    } else if let Some(url) = &input.url {
        Some(url.as_str())
    } else {
        None
    }
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let image = read_image(&args.input)
        .with_context(|| "read image data failed")?;
    let filename = get_filename(&args.input);
    print_image(&image, filename, &args.width, &args.height,
                args.preserve_aspect_ratio, &args.file_type);
    Ok(())
}
