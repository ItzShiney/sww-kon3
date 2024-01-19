use std::marker::PhantomData;

pub trait WgslBytesWriteable: encase::ShaderType + encase::internal::WriteInto {}
impl<T: encase::ShaderType + encase::internal::WriteInto + ?Sized> WgslBytesWriteable for T {}

pub struct WgslBytesWriter<T: WgslBytesWriteable + ?Sized> {
    buffer: Vec<u8>,
    phantom: PhantomData<T>,
}

impl<T: WgslBytesWriteable + ?Sized> Default for WgslBytesWriter<T> {
    fn default() -> Self {
        Self {
            buffer: Vec::default(),
            phantom: PhantomData,
        }
    }
}

impl<T: WgslBytesWriteable + ?Sized> WgslBytesWriter<T> {
    pub fn write(&mut self, value: &T) -> &[u8] {
        self.buffer.clear();

        let mut buffer = encase::StorageBuffer::new(&mut self.buffer);
        buffer.write(value).unwrap();

        &self.buffer
    }

    pub fn into_buffer(self) -> Vec<u8> {
        self.buffer
    }
}

pub fn to_wgsl_bytes<T: WgslBytesWriteable + ?Sized>(value: &T) -> Vec<u8> {
    let mut res = WgslBytesWriter::default();
    res.write(value);
    res.into_buffer()
}
