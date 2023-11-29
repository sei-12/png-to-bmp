use std::{env::args, collections::VecDeque};

use bmp::write_bmp_file;
use err::Err;
use filter::apply_filter;
use pngfile::{read_png_file, parse_idat};
use reader::WrapBufReader;
use miniz_oxide::inflate::decompress_to_vec_zlib;

mod pngfile;
mod ihdr;
mod err;
mod reader;
mod filter;
mod bmp;
mod color;

struct Arg {
    input_file_path: String,
    output_file_path: String
}

fn parse_args()-> Result<Arg,Err> {
    let mut args : VecDeque<String> = args().collect();
    args.pop_front();

    let input_file_path = args.pop_front().ok_or(Err::InvalidArg)?;
    // let output_file_path = args.pop_front().ok_or(Err::InvalidArg)?;
    let output_file_path = "output.bmp".to_string();

    Ok(Arg { input_file_path, output_file_path })
}


/**
 * 実質的なmain関数
 * mainで?をすると余分なエラーメッセージが出力されるから
 */
fn run()->Result<(),Err>{
    let arg = parse_args()?;
    let mut reader = WrapBufReader::new(arg.input_file_path)?;
    let (png_info,idat) = read_png_file(&mut reader)?;
    let decompressed_idat = decompress_to_vec_zlib(&idat)?;
    let (mut parsed_idat,filter_types) = parse_idat(&png_info, decompressed_idat)?;
    apply_filter(&mut parsed_idat, filter_types, &png_info)?;
    write_bmp_file(&png_info,parsed_idat, arg.output_file_path)?;
    Ok(())
}

fn main() {
    match run() {
        Err(err) => eprintln!("{}",err),
        _ => ()
    }
}