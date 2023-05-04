# Unpack Blend Textures

This is a Rust program that extracts texture images packed in Blender (*.blend) file and saves them as separate image files in a specified output directory.

## Installation

I provide binaries for Windows and macOS in the [Releases page](https://github.com/SamuelTallet/unpack-blend-textures/releases).

If you target Linux or another operating system:
1. [Install Rust](https://www.rust-lang.org/tools/install).
2. Compile the program with `cargo` as usual.

## Usage

To use the program, run the following command:

```shell
unpack-blend-textures <input.blend> <output-textures-dir>
```

where `<input.blend>` is the path to the input Blender file and `<output-textures-dir>` is the path to the output directory where the texture images will be extracted.

For example, to extract the textures from the "example.blend" file and save them to the "textures" directory, run the following command:

```shell
unpack-blend-textures "example.blend" "textures"
```

## Limitations

Only JPEG (*.jpg, *.jpeg) textures images are supported for now.

## Credits

This program is nothing without the [Rust blend crate](https://crates.io/crates/blend) by [Lucas Bittencourt](https://github.com/lukebitts).

## License

Licensed under the GPLv3. See the LICENSE file for details.

## Copyright

© 2023 Samuel Tallet
