use image::EncodableLayout;
use std::io::BufRead;
use std::io::Seek;
use wgpu::util::DeviceExt;

pub fn read_image(reader: impl BufRead + Seek) -> image::RgbaImage {
    image::io::Reader::new(reader)
        .with_guessed_format()
        .expect("failed to guess texture format")
        .decode()
        .expect("failed to decode texture")
        .into_rgba8()
}

pub fn read_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    reader: impl BufRead + Seek,
) -> wgpu::Texture {
    make_texture(device, queue, &read_image(reader))
}

pub fn make_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    image: &image::RgbaImage,
) -> wgpu::Texture {
    device.create_texture_with_data(
        queue,
        &wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::MipMajor,
        image.as_bytes(),
    )
}

pub fn make_default_texture(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Texture {
    make_texture(
        device,
        queue,
        &image::RgbaImage::from_pixel(1, 1, image::Rgba([255; 4])),
    )
}
