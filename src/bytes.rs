use encase::internal::BufferRef;

pub trait WgslBytesWriteable: encase::ShaderType + encase::internal::WriteInto {}
impl<T: encase::ShaderType + encase::internal::WriteInto + ?Sized> WgslBytesWriteable for T {}

pub trait WgslBytesWriteableSized: WgslBytesWriteable + encase::ShaderSize {}
impl<T: WgslBytesWriteable + encase::ShaderSize + ?Sized> WgslBytesWriteableSized for T {}

pub trait WgslBytesReadable: encase::ShaderType + encase::internal::CreateFrom {}
impl<T: encase::ShaderType + encase::internal::CreateFrom + ?Sized> WgslBytesReadable for T {}

pub fn to_wgsl_bytes<T: WgslBytesWriteable + ?Sized>(value: &T) -> Vec<u8> {
    let mut res = Vec::default();

    let mut buffer = encase::StorageBuffer::new(&mut res);
    buffer.write(value).unwrap();

    res
}

pub fn from_wgsl_bytes<T: WgslBytesReadable, B: BufferRef>(bytes: B) -> T {
    encase::StorageBuffer::new(bytes).create().unwrap()
}
