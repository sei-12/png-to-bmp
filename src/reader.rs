use std::{io::BufReader, fs::File,io::Read};

use crate::err::Err;


pub struct WrapBufReader {
    inner: BufReader<File>
}

impl WrapBufReader {
    pub fn new(file_path:String) -> Result<Self,Err> {
        let inner = BufReader::new(File::open(file_path)?);
        Ok(WrapBufReader { inner })
    }
    
    pub fn read_u32(&mut self) -> Result<u32,Err> {
        let mut buf = [0;4];
        let size = self.inner.read(&mut buf)?;
        if size != 4 {
            todo!()// err
        }else{
            Ok(u32::from_be_bytes(buf))
        }
    }

    pub fn read_chunk_name(&mut self) -> Result<String,Err> {
        let mut buf = [0;4];
        let size = self.inner.read(&mut buf)?;
        if size != 4 {
            todo!()// err
        }else{
            let s = String::from_utf8_lossy(&buf).to_string();
            Ok(s)
        }
    }

    pub fn read_crc(&mut self) -> Result<(),Err> {
        let mut buf = [0;4];
        let size = self.inner.read(&mut buf)?;
        if size != 4 {
            todo!()// err
        }else{
            Ok(())
        }
    }

    pub fn read_file_signature(&mut self) -> Result<Box<[u8;8]>,Err> {
        let mut buf = Box::new([0;8]);
        let size = self.inner.read(&mut *buf)?;
        if size == 8 {
            Ok(buf)
        }else{
            todo!()// err
        }
    }

    pub fn read_len(&mut self,len:u32) -> Result<Vec<u8>,Err> {
        let mut buf = [0;1];
        let mut result = Vec::new();

        for _ in 0..len {
            let size = self.inner.read(&mut buf)?;
            if size != 1 { todo!() }
            result.push(buf[0]);
        }

        Ok(result)
    }
}