use std::{io::BufWriter, fs::File,io::Write};

use crate::{err::Err, pngfile::PngInfo, color::Color};

const BMP_FILE_TYPE : [ u8;2 ]= [ 0x42, 0x4d ];
const BITS_PER_BYTE : usize = 8;

struct WrapBufWriter {
    inner: BufWriter<File>
}

impl WrapBufWriter {
    fn new(file_path:String) -> Result<Self,Err> {
        let inner = BufWriter::new(File::create(file_path)?);
        Ok(WrapBufWriter { inner })
    }

    fn w_u32(&mut self,val:u32) -> Result<(),Err> {
        let arr : [u8;4] = unsafe { std::mem::transmute([val]) };
        self.inner.write(&arr)?;
        Ok(())
    }

    fn write(&mut self,val:&[u8]) -> Result<(),Err> {
        self.inner.write(val)?;
        Ok(())
    }
}

pub fn write_bmp_file(png_info:&PngInfo,image_data:Vec<Vec<Color>>,file_path:String) -> Result<(),Err> {
    let mut buf_writer = WrapBufWriter::new(file_path)?;

    write_file_header(&mut buf_writer,png_info)?;
    write_info_header(&mut buf_writer,png_info)?;
    write_bit_map_data(&mut buf_writer,image_data,png_info)?;

    Ok(())
}

fn calc_bmp_file_size(png_info:&PngInfo) -> u32 {
    offset() + calc_image_data_size(png_info)
}

fn offset() -> u32 {
    14 + 40
}

fn write_file_header(writer:&mut WrapBufWriter,png_info:&PngInfo) -> Result<(),Err>{
    writer.write(&BMP_FILE_TYPE)?;
    writer.w_u32(calc_bmp_file_size(png_info))?;
    writer.write(&[0x00,0x00,0x00,0x00])?; // 4バイトなにもない空間がある
    writer.w_u32(offset())?;
    Ok(())
}

fn calc_image_data_size(png_info:&PngInfo) -> u32 {
    png_info.height * png_info.width * png_info.bit_per_px as u32 / BITS_PER_BYTE as u32
}

// 参考: https://ja.wikipedia.org/wiki/Windows_bitmap#:~:text=%E3%82%92%E9%99%A4%E3%81%8F%E3%81%93%E3%81%A8%EF%BC%89-,BITMAPINFOHEADER,-%5B%E7%B7%A8%E9%9B%86%5D
fn write_info_header(writer:&mut WrapBufWriter,png_info:&PngInfo) -> Result<(),Err> {
    // header size
    writer.w_u32(40)?;
    
    writer.w_u32(png_info.width)?;
    writer.w_u32(png_info.height)?;

    // of plane 常に1
    writer.write(&[0x01,0x00])?;
    // 1ピクセルあたりのビット数
    writer.write(&[24,0])?;

    // 圧縮方式 (圧縮しないから常に0)
    writer.w_u32(0)?;
    
    // 画像データサイズ
    writer.w_u32(calc_image_data_size(png_info))?;

    // 解像度の情報はなくても大丈夫みたい めんどくさいので0いれとく
    writer.w_u32(0)?; // horizontal
    writer.w_u32(0)?; // vertical

    // ビットマップで実際に使用するカラーパレット内のカラーインデックスの数。
    writer.w_u32(0)?;
    // ビットマップを表示するために必要なカラーインデックスの数。
    writer.w_u32(0)?;
    
    Ok(())
}

fn calc_padding_size(row_length:usize) -> usize {
    (4 - (row_length % 4)) % 4
}

fn write_padding(writer:&mut WrapBufWriter,size:usize) -> Result<(),Err> {
    for _ in 0..size {
        writer.write(&[0x00])?;
    };
    Ok(())
}

fn calc_row_bytes(png_info:&PngInfo) -> usize {
    const PADDING : usize = BITS_PER_BYTE - 1;
    let width = png_info.width as usize;

    ( png_info.bit_per_px * width + PADDING ) / BITS_PER_BYTE
}

fn write_bit_map_data(writer:&mut WrapBufWriter,mut image_data:Vec<Vec<Color>>,png_info:&PngInfo) -> Result<(),Err>{
    let row_size = calc_row_bytes(png_info);
    let padding_size = calc_padding_size(row_size);

    loop {
        let Some(row) = image_data.pop() else { break; };

        for c in row {
            writer.write(&[c.b(&png_info.color_type)])?;
            writer.write(&[c.g(&png_info.color_type)])?;
            writer.write(&[c.r(&png_info.color_type)])?;
        }

        write_padding(writer,padding_size)?;
    }

    Ok(())
}