
use miniz_oxide::inflate::DecompressError;
use thiserror::Error;

#[derive(Debug,Error)]
pub enum Err {
    #[error("invalid argument")]
    InvalidArg,

    #[error("not png file")]
    NotPngFile,

    #[error("unknown bit depth : {0}")]
    UnknownBitDepth(u8),
    #[error("unknown color type : {0}")]
    UnknownColorType(u8),
    #[error("unknown compress method : {0}")]
    UnknownCompressMethod(u8),
    #[error("unknown filter method : {0}")]
    UnknownFilterMethod(u8),
    #[error("unknown Interlace method : {0}")]
    UnknownInterlaceMethod(u8),
    
    #[error("unknown filter type : {0}")]
    UnknownFilterType(u8),

    #[error("unsupport chunk: {0}")]
    UnsupportChunk(String),

    #[error(transparent)]
    IO {
        #[from]
        source: std::io::Error
    },

    #[error("DecompressError")]
    Decompress,

    #[error("todo")]
    TODO
}


impl From<DecompressError> for Err {
    fn from(_value: DecompressError) -> Self {
        Err::Decompress
    }
}