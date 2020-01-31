use std::convert::TryFrom;
use std::os::raw::c_int;

mod ffi;

/// Decodes a WebP image and returns RGBA samples, along with the image
/// dimensions.
///
/// The ordering of samples in memory is R, G, B, A, R, G, B, A... in scan order
/// (endian-independent). Returns `None` in case of error.
pub fn decode_rgba(data: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let success = unsafe { ffi::WebPGetInfo(data.as_ptr(), data.len(), &mut width, &mut height) };

    if success == 0 {
        return None;
    }

    if width == 0 || height == 0 || width.is_negative() || height.is_negative() {
        return None;
    }

    let width_usize = usize::try_from(width).ok()?;
    let height_usize = usize::try_from(height).ok()?;
    let width_u32 = u32::try_from(width).ok()?;
    let height_u32 = u32::try_from(height).ok()?;

    // FIXME: @Optimization Don't zero the memory, live dangerously.
    let output_buffer_size = width_usize * height_usize * 4;
    let mut output_buffer: Vec<u8> = vec![0; output_buffer_size];

    let output_stride = width * 4;

    let result = unsafe {
        ffi::WebPDecodeRGBAInto(
            data.as_ptr(),
            data.len(),
            output_buffer.as_mut_ptr(),
            output_buffer.len(),
            output_stride,
        )
    };

    if result.is_null() {
        return None;
    }

    Some((width_u32, height_u32, output_buffer))
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::path::Path;

    use super::*;

    fn load_and_decode_png<P: AsRef<Path>>(
        path: P,
    ) -> Result<(u32, u32, Vec<u8>), png::DecodingError> {
        let file = File::open(path)?;
        let decoder = png::Decoder::new(file);
        let (info, mut reader) = decoder.read_info()?;

        let mut data = vec![0; info.buffer_size()];
        reader.next_frame(&mut data)?;

        assert_eq!(info.color_type, png::ColorType::RGBA);
        assert_eq!(info.bit_depth, png::BitDepth::Eight);

        Ok((info.width, info.height, data))
    }

    #[test]
    fn test_160x160_rgba() {
        let webp = fs::read("tests/fixtures/bottle_160x160_rgba.webp").unwrap();
        let (webp_width, webp_height, webp_data) = decode_rgba(&webp).unwrap();
        let (png_width, png_height, png_data) =
            load_and_decode_png("tests/fixtures/bottle_160x160_rgba_converted.png").unwrap();

        assert_eq!(webp_width, png_width);
        assert_eq!(webp_height, png_height);
        assert_eq!(webp_data, png_data);
    }

    #[test]
    fn test_invalid() {
        let webp = fs::read("tests/fixtures/invalid.webp").unwrap();
        let option = decode_rgba(&webp);

        assert_eq!(option, None);
    }
}
