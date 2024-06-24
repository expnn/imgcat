# imgcat

## Overview

`imgcat` is a command-line tool that allows you to display images inline in terminals that support iTerm2's Inline Images Protocol. 
This tool is useful for quickly visualizing images directly in your terminal, without the need to open an external image viewer.

## Features

- **Automatic Sizing:** If you don't specify the width or height, `imgcat` will choose an appropriate value automatically.
- **Flexible Sizing Options:** You can specify the width and height in character cells, pixels, or as a percentage of the terminal's dimensions.
- **File Type Hinting:** Provide a file type to help disambiguate the content, especially when input comes from a pipe.
- **Preserve Aspect Ratio:** Option to stretch the image while preserving the aspect ratio.
- **Print Image Path:** Option to print the path or URL of the image.

## Usage

The basic syntax for using `imgcat` is:

```
imgcat [OPTIONS] [INPUTS]...
```

### Arguments

- `[INPUTS]...`  
  Input image files or URLs to show. If not provided, `imgcat` reads from stdin.

### Options

- `-t, --file-type <FILE_TYPE>`  
  Specify the file type. This can be a MIME type (e.g., `image/png`), a language name (e.g., `Java`), or a file extension (e.g., `.c`). This is particularly useful when the filename is not available.

- `-W, --width <WIDTH>`  
  Set the output width of the image. The width can be specified in character cells (e.g., `40`), pixels (e.g., `250px`), or as a percentage of the terminal's width (e.g., `100%`).

- `-H, --height <HEIGHT>`  
  Set the output height of the image. The height can be specified similarly to the width.

- `-s, --stretch`  
  Preserve the aspect ratio when drawing the image.

- `-p, --print-path`  
  Print the path or URL of the image.

- `-h, --help`  
  Print the help message.

- `-V, --version`  
  Print the version information.

## Examples

Display an image with specified width and height:

```sh
$ imgcat -W 250px -H 250px -s avatar.png
```

Display an image from a pipe with full terminal width:

```sh
$ cat graph.png | imgcat -W 100%
```

Print the image path and display an image from a URL with specified width:

```sh
$ imgcat -p -W 500px -u http://host.tld/path/to/image.jpg -W 80 -f image.png
```

Use `xargs` to display images from a list of URLs with specified width:

```sh
$ cat url_list.txt | xargs imgcat -p -W 40 -u
```

Specify the file type when displaying a JSON file:

```sh
$ imgcat -t application/json config.json
```

## Installation

To install `imgcat`, follow these steps:

1. Download the binary from the [releases page](https://github.com/expnn/imgcat/releases).
2. Place the binary in a directory that is in your PATH.
3. Make the binary executable:
   ```sh
   chmod +x /path/to/imgcat
   ```

## Support

For support, please open an issue on the [GitHub repository](https://github.com/expnn/imgcat/issues).

## License

`imgcat` is licensed under the MIT License. See the LICENSE file for more information.
