use std::{vec::IntoIter, collections::VecDeque};

use crate::{reader::WrapBufReader, err::Err, ihdr::{IHDR, ColorType, BitDepth}, color::Color};

const PNG_SIGNATURE : [u8;8] = [ 0x89,0x50,0x4e,0x47,0x0d,0x0a,0x1a,0x0a ];

pub struct PngInfo {
    pub width: u32,
    pub height: u32,
    pub color_type: ColorType,
    pub bit_per_px: usize,
}

impl PngInfo {
    fn new(ihdr:IHDR) -> Self {
        PngInfo { 
            width: ihdr.width, 
            height: ihdr.height, 
            bit_per_px: PngInfo::calc_bits_per_px(&ihdr.color_type, &ihdr.bit_depth), 
            color_type: ihdr.color_type, 
        }
    }

    fn calc_bits_per_px(color_type:&ColorType,bit_depth:&BitDepth) -> usize {
        match color_type {
            ColorType::RGB => (bit_depth.val * 3) as usize,
            ColorType::RGBA => (bit_depth.val * 4) as usize
        }
    }
}

#[allow(non_camel_case_types)]
pub enum Chunk {
    IHDR(IHDR),
    IDAT(Vec<u8>),
    IEND,
    sRGB(),
    tEXt(String),
    Unknown{ name:String }
}
impl Chunk {
    pub fn new(name:String,data:Vec<u8>) -> Result<Self,Err> {
        if name == "IHDR" {
            return Ok(Self::IHDR(IHDR::new(data)?));
        };
        if name == "IDAT" {
            return Ok(Self::IDAT(data));
        };
        if name == "IEND" {
            return Ok(Self::IEND);
        };
        if name == "sRGB" {
            return Ok(Self::sRGB());
        };
        if name == "tEXt" {
            let text = match String::from_utf8(data) {
                Ok(data) => data,
                Err(_) => format!("parse error")
            };
            return Ok(Self::tEXt(text));
        };
        
        if name.len() != 4 {
            return Err(Err::TODO);
        };

        if name.chars().nth(0).ok_or(Err::TODO)?.is_uppercase() {
            return Err(Err::UnsupportChunk(name));
        };

        Ok(Self::Unknown { name })
    }
}

pub fn read_png_file(reader:&mut WrapBufReader) -> Result<(PngInfo,Vec<u8>),Err>{
    let sig = reader.read_file_signature()?;
    if *sig != PNG_SIGNATURE { return Err(Err::NotPngFile);}

    let mut chunks = VecDeque::new();

    loop {
        let length = reader.read_u32()?;
        let chunk_name = reader.read_chunk_name()?;
        let chunk_data = reader.read_len(length)?;
        reader.read_crc()?;
        chunks.push_back(Chunk::new(chunk_name, chunk_data)?);
        if matches!(chunks.back(),Some(&Chunk::IEND)) { break; }
    }

    let png_info = match chunks.pop_front().ok_or(Err::TODO)? {
        Chunk::IHDR(ihdr) => PngInfo::new(ihdr),
        _ => { return Err(Err::TODO); }
    };

    let mut idat = Vec::new();

    for chunk in chunks.into_iter() {
        match chunk {
            Chunk::IDAT(mut data) => idat.append(&mut data),
            _ => ()
        }
    };

    Ok((png_info,idat))
}

fn iter_num_next<T>(iter:&mut IntoIter<T>,len:usize) -> Result<Vec<T>,Err> {
    let mut result = Vec::with_capacity(len);

    for _ in 0..len {
        let Some(val) = iter.next() else {
            todo!();

        };

        result.push(val);
    }

    Ok(result)
}

pub fn parse_idat(png_info:&PngInfo,data:Vec<u8>) -> Result<(Vec<Vec<Color>>,Vec<u8>),Err> {
    let mut filter_types: Vec<u8> = Vec::new();
        
    let mut image_data : Vec<Vec<Color>> = Vec::with_capacity(png_info.height as usize);
    
    let mut data_iter = data.into_iter();
    let byte_per_px = png_info.bit_per_px / 8 ;

    for _ in 0..png_info.height {
        match data_iter.next() {
            Some(filter_type) => filter_types.push(filter_type),
            None => todo!()
        }

        let mut row: Vec<Color> = Vec::with_capacity(png_info.width as usize);
        for _ in 0..png_info.width {
            let color_data = iter_num_next(&mut data_iter, byte_per_px)?;
            let color = Color::new(color_data);
            row.push(color);
        }

        image_data.push(row);
    }
    
    Ok((image_data,filter_types))
}