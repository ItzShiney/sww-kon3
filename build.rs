use std::fs;

fn main() {
    for file_name in ["mesh"] {
        let input_path = format!("src/{}.wgsl", file_name);
        let output_path = format!("src/shaders/{}.rs", file_name);
        let input_absolute_path = String::from("../../") + &input_path;

        cargo_emit::rerun_if_changed!(input_path);

        let input = fs::read_to_string(&input_path).unwrap();

        let options = wgsl_to_wgpu::WriteOptions::default();
        let text =
            wgsl_to_wgpu::create_shader_module(&input, &input_absolute_path, options).unwrap();

        fs::write(output_path, text.as_bytes()).unwrap();
    }
}
