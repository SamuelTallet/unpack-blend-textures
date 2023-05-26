use std::env;
use std::path::Path;
use std::fs::metadata as file_meta;
use std::fs::create_dir;
use std::fs::File;
use std::io::Write;
use blend::Blend;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: ./unpack-blend-textures <input.blend> <output-textures-dir>");
        return;
    }

    let input_blend_path = &args[1];
    println!("Input .blend file: {}", input_blend_path);

    let blend = Blend::from_path(input_blend_path)
        .expect("Error loading input .blend file");

    let output_textures_dir = &args[2];
    println!("Output textures dir: {}", output_textures_dir);

    if !file_meta(output_textures_dir).is_ok() {
        create_dir(output_textures_dir)
            .expect("Error creating output textures dir.");
    }

    for image in blend.instances_with_code(*b"IM") {
        let image_name = image.get_string("name");

        if !image.fields.contains_key("packedfile") || !image_name.starts_with("//") {
            println!("Skipping unexpected image instance");
            continue;
        }

        let local_image_name = image_name.strip_prefix("//").unwrap();
        let canonical_image_name = local_image_name.replace("\\", "/");
        let image_filename = Path::new(&canonical_image_name)
            .file_name()
            .map(|filename| filename.to_string_lossy().into_owned())
            .unwrap();

        let image_packed_file = image.get("packedfile");
        let image_data = image_packed_file.get_u8_vec("data");

        println!("Unpacking texture: {}", image_filename);

        let mut image_file = File::create(format!("{}/{}", output_textures_dir, image_filename))
            .expect("Error creating texture image file");
        image_file.write_all(&image_data)
            .expect("Error writing texture image file");

    }
}
