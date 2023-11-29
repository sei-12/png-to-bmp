

use std::vec::IntoIter;

use crate::err::Err;

#[derive(Debug)]
pub struct BitDepth {
    pub val:u8
}
impl BitDepth {
    fn from_byte(b:u8) -> Result<Self,Err> {
        if b == 8 { Ok(BitDepth{val:b}) }
        else { Err(Err::UnknownBitDepth(b)) }
    }
}

#[derive(Debug,Clone)]
pub enum ColorType {
    // GrayScale, // 0
    RGB, // 2
    // IndexColor, // 3
    // GarayAlpha, // 4
    RGBA, // 6
}
impl ColorType {
    fn from_byte(b:u8) -> Result<Self,Err> {
        match b {
            2 => Ok(Self::RGB),
            6 => Ok(Self::RGBA),
            _ => Err(Err::UnknownColorType(b))
        }
    }
}

#[derive(Debug)]
pub enum InterlaceMethod {
    None,
    // Adam7
}
impl InterlaceMethod {
    fn from_byte(b:u8) -> Result<Self,Err> {
        match b {
            0 => Ok(Self::None),
            _ => Err(Err::UnknownInterlaceMethod(b))
        }
    }
}


pub struct IHDR {
    pub width: u32,
    pub height: u32,
    pub bit_depth: BitDepth,
    pub color_type: ColorType,
    pub interlace_method: InterlaceMethod
    /*
        圧縮方式とフィルタ方式は1つしかないから、チェックだけして、保持はしない
    */
}

fn next_u32(data:&mut IntoIter<u8>) -> Result<u32,Err> {
    let mut buf = [0;4];
    for i in 0..4 {
        buf[i] = data.next().ok_or(Err::TODO)?;
    };
    Ok(u32::from_be_bytes(buf))
}

fn next_u8(data:&mut IntoIter<u8>) -> Result<u8,Err> {
    data.next().ok_or(Err::TODO)
}

#[inline]
fn check_compress_method(b:u8) -> Result<(),Err> {
    if b != 0 {
        Err(Err::UnknownCompressMethod(b))
    }else{
        Ok(())
    }
}

#[inline]
fn check_filter_method(b:u8) -> Result<(),Err> {
    if b != 0 {
        Err(Err::UnknownFilterMethod(b))
    }else{
        Ok(())
    }
}

impl IHDR {
    pub fn new(data:Vec<u8>) -> Result<Self,Err> {
        let mut data = data.into_iter();

        let width  = next_u32(&mut data)?;
        let height = next_u32(&mut data)?;
        let bit_depth = BitDepth::from_byte(next_u8(&mut data)?)?;
        let color_type = ColorType::from_byte(next_u8(&mut data)?)?;
        check_compress_method(next_u8(&mut data)?)?;
        check_filter_method(next_u8(&mut data)?)?;
        let interlace_method = InterlaceMethod::from_byte(next_u8(&mut data)?)?;

        Ok(IHDR { width, height, bit_depth, color_type, interlace_method })
    }
}