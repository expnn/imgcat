use std::fs::{self, File};
use std::{env, io};
use std::io::Read;
use anyhow;
use anyhow::Context;
use base64::Engine;
use base64::prelude::{BASE64_STANDARD, BASE64_URL_SAFE};
use clap::{Parser, ArgAction, arg};
use reqwest;
use url::Url;
use pathsep::path_separator;
use phf::{phf_set, Set};

const SUPPORTED_SCHEMES: Set<&'static str> = phf_set!{
     "http", "https", "ftp",
};

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
/// The file type can be a mime type like text/markdown, a language name like Java, or a file extension like .c
/// The file type can usually be inferred from the extension or its contents. -t is most useful when"
/// a filename is not available, such as whe input comes from a pipe."
///
/// Examples:
///
///     $ imgcat -W 250px -H 250px -s avatar.png
///     $ cat graph.png | imgcat -W 100%
///     $ imgcat -p -W 500px -u http://host.tld/path/to/image.jpg -W 80 -f image.png
///     $ cat url_list.txt | xargs imgcat -p -W 40 -u
///     $ imgcat -t application/json config.json
#[derive(Parser, Debug)]
#[command(version, about, long_about, verbatim_doc_comment)]
struct Cli {
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

    /// whether to print the path or URL of the image or not
    #[arg(short, long)]
    print_path: bool,

    /// input image files or URLs to show. Read from stdin if not given
    #[arg(num_args = 0..)]
    inputs: Vec<String>
}

struct Image<'a> {
    data: Vec<u8>,
    filename: Option<String>,
    path: Option<&'a str>,
}

impl<'a> Image<'a> {
    fn try_new(path: &'a str) -> anyhow::Result<Self> {
        // 由于在 Windows 中， 类似 C:/a/b/c 这样的绝对路径可以被 Url::parse 函数正确解析。
        // 这里限定 scheme 为给定集合中的值时，才认为他是一个图片的 URL。
        if let Ok(u) = Url::parse(path) {
            if SUPPORTED_SCHEMES.contains(u.scheme()) {
                let filename = u.path()
                    .trim_end_matches('/')
                    .rsplitn(2, '/')
                    .next()
                    .map(|x| x.to_string());
                let data = reqwest::blocking::get(u)
                    .with_context(|| format!("failed to connect to {path}"))?
                    .bytes()
                    .with_context(|| format!("failed to fetch image data from {path}"))?
                    .iter()
                    .cloned()
                    .collect();
                return Ok(Self {data, filename, path: Some(path)});
            }
        }

        // 其余情况，包括 Url 解析出错，或者解析得到的 scheme 不在给定的集合中，
        // 则回退到认为给定的 path 是一个本地文件系统的路径。
        let f = path.trim_start_matches("file://");
        let filename = f.rsplitn(2, path_separator!())
            .next()
            .map(|x| x.to_string());
        let mut file = File::open(path)
            .with_context(|| format!("failed to open file {f}"))?;
        let metadata = fs::metadata(&f);
        let mut buffer = match metadata {
            Ok(m) => {vec![0; m.len() as usize]}
            Err(_) => {Vec::new()}
        };
        file.read(&mut buffer)
            .with_context(|| format!("failed to read from file {f}"))?;
        Ok(Self {data: buffer, filename, path: Some(path)})
    }

    fn from_stdin() -> anyhow::Result<Self> {
        let mut data = Vec::new();
        io::stdin().read_to_end(&mut data)
            .with_context(|| "failed to read stdin")?;
        Ok(Self {data, filename: None, path: None})
    }

    fn len(&self) -> usize {
        self.data.len()
    }
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
    image: Image,
    args: &Cli,
) {
    print_osc();
    print!("1337;File=inline=1;size={}", image.len());

    if let Some(name) = &image.filename {
        print!(";name={}", BASE64_URL_SAFE.encode(name));
    }

    if let Some(w) = &args.width {
        print!(";width={w}");
    }

    if let Some(h) = &args.height {
        print!(";height={h}");
    }

    print!(";preserveAspectRatio={}", args.preserve_aspect_ratio as u8);

    if let Some(ft) = &args.file_type {
        print!(";type={ft}");
    }
    print!(":{}", BASE64_STANDARD.encode(&image.data));
    print_st();

    println!();
    if args.print_path {
        if let Some(name) = &image.path {
            println!("{name}");
        }
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

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    println!("got {} input images", args.inputs.len());
    if args.inputs.is_empty() {
        let image = Image::from_stdin()?;
        print_image(image, &args);
    } else {
        args.inputs
            .iter()
            .try_for_each(|x| -> anyhow::Result<()> {
                print_image(Image::try_new(x)?, &args);
                Ok(())
            })?;
    }
    Ok(())
}
