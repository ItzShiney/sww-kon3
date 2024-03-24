use std::fs;

fn main() {
    #[allow(clippy::single_element_loop)]
    for file_name in ["mesh"] {
        let input_path = format!("src/{}.wgsl", file_name);
        let output_path = format!("src/shaders/{}.rs", file_name);
        let input_absolute_path = String::from("../../") + &input_path;

        cargo_emit::rerun_if_changed!(input_path);
        cargo_emit::rerun_if_changed!(output_path);
        let input = fs::read_to_string(&input_path).unwrap();

        let options = wgsl_to_wgpu::WriteOptions {
            derive_encase: true,
            matrix_vector_types: wgsl_to_wgpu::MatrixVectorTypes::Glam,
            ..Default::default()
        };

        let text =
            wgsl_to_wgpu::create_shader_module(&input, &input_absolute_path, options).unwrap();
        let text = text.replace("memoffset", "std::mem");

        fs::write(output_path, text.as_bytes()).unwrap();
    }
}
