use std::env;
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
        let image_name = image.get("id").get_string("name");
        let mut cleaned_image_name = image_name.trim_start_matches("IM").to_owned();

        for i in 1..=100 {
            let image_suffix = format!(".{:03}", i);
            if cleaned_image_name.ends_with(&image_suffix) {
                cleaned_image_name = cleaned_image_name.trim_end_matches(&image_suffix).to_owned();
                break;
            }
        }

        if image.fields.contains_key("packedfile") &&
            (cleaned_image_name.ends_with(".jpg") || cleaned_image_name.ends_with(".jpeg")) {

            let image_packed_file = image.get("packedfile");

            if image_packed_file.fields.contains_key("data") {
                let image_data = image_packed_file.get_u8_vec("data");

                println!("Unpacking texture: {}", cleaned_image_name);

                let mut image_file = File::create(format!("{}/{}", output_textures_dir, cleaned_image_name))
                    .expect("Error creating texture image file");
                image_file.write_all(&image_data)
                    .expect("Error writing texture image file");

            }

        }

    }
}