//! Entry point for all supported codecs  
//! the library contains
//!
//! Current status
//!
//! |IMAGE| Decoder     |Encoder|
//! |-----|-------------|-------|
//! |JPEG |Full support| None |
//! |PNG |Partial  |None |
//! |PPM | 8 and 16 bit support |8 and 16 bit support|
//! |PAL | None |8 and 16 bit support |
//!
//!
#[allow(unused_imports)]
use crate::traits::DecoderTrait;

pub mod jpeg;
pub mod png;
pub mod ppm;

/// All supported decoders
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SupportedDecoders
{
    /// Fully complete
    Jpeg,
    /// Not yet complete
    Png,
    /// Fully complete
    PPM
}

/// All supported encoders
#[derive(Debug)]
pub enum SupportedEncoders
{
    PPM,
    //PPM encoder
    PAM // PAM encoder
}

// stolen from imagers
static MAGIC_BYTES: [(&[u8], SupportedDecoders); 4] = [
    (&[137, 80, 78, 71, 13, 10, 26, 10], SupportedDecoders::Png),
    (&[0xff, 0xd8, 0xff], SupportedDecoders::Jpeg),
    (b"P6", SupportedDecoders::PPM),
    (b"P5", SupportedDecoders::PPM)
];
/// Return the format of an image or none if it's unsupported
pub fn guess_format(bytes: &[u8]) -> Option<SupportedDecoders>
{
    for (magic, decoder) in MAGIC_BYTES
    {
        if bytes.starts_with(magic)
        {
            return Some(decoder);
        }
    }
    None
}

/// Get a decoder capable of decoding `codec` bytes represented by `data`
///
/// This does not handle special form decoders, i.e it uses default settings
/// for decoders
#[cfg(any(feature = "png", feature = "jpeg"))]
pub fn get_decoder<'a>(codec: SupportedDecoders, data: &'a [u8]) -> Box<dyn DecoderTrait + 'a>
{
    match codec
    {
        SupportedDecoders::Jpeg =>
        {
            #[cfg(feature = "jpeg")]
            {
                Box::new(zune_jpeg::JpegDecoder::new(data))
            }
            #[cfg(not(feature = "jpeg"))]
            {
                unimplemented!("JPEG feature not included")
            }
        }

        SupportedDecoders::Png =>
        {
            #[cfg(feature = "png")]
            {
                Box::new(zune_png::PngDecoder::new(data))
            }
            #[cfg(not(feature = "png"))]
            {
                unimplemented!("PNG feature not included")
            }
        }
        SupportedDecoders::PPM =>
        {
            #[cfg(feature = "ppm")]
            {
                Box::new(zune_ppm::PPMDecoder::new(data))
            }
            #[cfg(not(feature = "ppm"))]
            {
                unimplemented!("PPM feature not included")
            }
        }
    }
}
