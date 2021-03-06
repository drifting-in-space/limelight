use limelight::{
    webgl::types::{DataType, SizedDataType},
    AsSizedDataType,
};
use palette::{Srgb, Srgba};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Color(pub u32);

impl Color {
    pub fn opacity(&self, opacity: f32) -> Self {
        let opacity = ((255. * opacity) as u32).min(255);
        Color(self.0 & (0x00ffffff + (opacity << 24)))
    }
}

impl AsSizedDataType for Color {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType::new(DataType::UnsignedInt, 1)
    }
}

impl From<Srgb<u8>> for Color {
    fn from(srgb: Srgb<u8>) -> Self {
        Color(*bytemuck::from_bytes(&[
            srgb.red, srgb.green, srgb.blue, 0xff,
        ]))
    }
}

impl From<Srgba<u8>> for Color {
    fn from(srgba: Srgba<u8>) -> Self {
        Color(*bytemuck::from_bytes(&[
            srgba.red,
            srgba.green,
            srgba.blue,
            srgba.alpha,
        ]))
    }
}
