use crate::{pngfile::PngInfo, err::Err, color::Color};

pub fn apply_filter(idat:&mut Vec<Vec<Color>>,filter_types:Vec<u8>,png_info:&PngInfo) -> Result<(),Err>{
        
    for y in 0..png_info.height as usize {
        let filter_type = filter_types[y];

        match filter_type {
            0 => { continue; },
            1 => apply_sub(idat, y, png_info)?,
            2 => apply_up(idat, y, png_info)?,
            3 => apply_avg(idat, y, png_info)?,
            4 => apply_paeth(idat, y, png_info)?,
            _ => return Err(Err::UnknownFilterType(filter_type))
        };
    }
    
    Ok(())
}

#[inline]
fn apply_sub(idat:&mut Vec<Vec<Color>>,y:usize,png_info:&PngInfo) -> Result<(),Err>{
    for x in 0..png_info.width as usize {
        let target          = data_get(&idat,y, x).ok_or(Err::TODO)?;
        let left    = left   (&idat,y, x);
        idat[y][x] = target.apply_sub(left);
    }
    Ok(())
}

#[inline]
fn apply_up(idat:&mut Vec<Vec<Color>>,y:usize,png_info:&PngInfo) -> Result<(),Err> {
    for x in 0..png_info.width as usize {
        let target = data_get(&idat,y, x).ok_or(Err::TODO)?;
        let up = up(&idat,y, x);
        idat[y][x] = target.apply_up(up);
    }
    Ok(())
}

#[inline]
fn apply_avg(idat:&mut Vec<Vec<Color>>,y:usize,png_info:&PngInfo) -> Result<(),Err> {
    for x in 0..png_info.width as usize {
        let target          = data_get(&idat,y, x).ok_or(Err::TODO)?;
        let left    = left   (&idat,y, x);
        let up      = up     (&idat,y, x);
        idat[y][x] = target.apply_avg(left, up);
    };
    Ok(())
}

#[inline]
fn apply_paeth(idat:&mut Vec<Vec<Color>>,y:usize,png_info:&PngInfo) -> Result<(),Err> {
    for x in 0..png_info.width as usize {
        let target          = data_get(&idat,y, x).ok_or(Err::TODO)?;
        let left    = left   (&idat,y, x);
        let up      = up     (&idat,y, x);
        let left_up = left_up(&idat,y, x);
        idat[y][x] = target.apply_paeth(left, up, left_up);
    }
    Ok(())
}

fn left(idat:&Vec<Vec<Color>>,y:usize,x:usize) -> Option<&Color> {
    if x > 0 {
        idat.get(y)?.get(x - 1)
    }else{
        None
    }
}

fn up(idat:&Vec<Vec<Color>>,y:usize,x:usize) -> Option<&Color>{
    if y > 0 {
        idat.get(y - 1)?.get(x)
    }else{
        None
    }
}

fn left_up(idat:&Vec<Vec<Color>>,y:usize,x:usize) -> Option<&Color>{
    if y > 0 && x > 0 {
        idat.get(y - 1)?.get(x - 1 )
    }else{
        None
    }
}

#[inline]
fn data_get(idat:&Vec<Vec<Color>>,y:usize,x:usize) -> Option<&Color> {
    idat.get(y)?.get(x)
}