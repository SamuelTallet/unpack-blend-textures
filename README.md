# Unpack Blend Textures

This program extracts the texture images packed in a Blender (*.blend) file and saves them as separate image files in a specified directory.

Written in Rust, it is very fast!

## Installation

I provide binaries for Windows and macOS in the [Releases page](https://github.com/SamuelTallet/unpack-blend-textures/releases).

If you target Linux or another system:
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

This project would have never existed without the [Rust blend crate](https://crates.io/crates/blend) made by [Lucas Bittencourt](https://github.com/lukebitts).

## License

Program licensed under the GPLv3. See the LICENSE file for details.

## Copyright

© 2023 Samuel Tallet
