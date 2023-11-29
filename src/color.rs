use crate::ihdr::ColorType;



pub struct Color {
    data: Vec<u8>
}

impl Color {
    pub fn new(data:Vec<u8>) -> Self {
        Color { data }
    }

    pub fn r(&self,color_type:&ColorType) -> u8 {
        match color_type {
            ColorType::RGB => self.data[0],
            ColorType::RGBA => self.data[0]
        }
    }

    pub fn g(&self,color_type:&ColorType) -> u8 {
        match color_type {
            ColorType::RGB => self.data[1],
            ColorType::RGBA => self.data[1]
        }
    }

    pub fn b(&self,color_type:&ColorType) -> u8 {
        match color_type {
            ColorType::RGB => self.data[2],
            ColorType::RGBA => self.data[2]
        }
    }

    pub fn apply_sub(&self,left:Option<&Color>) -> Color {
        let mut new_data = Vec::with_capacity(self.data.len());

        let zeros = vec![0;self.data.len()];
        let left_datas = if let Some(left) = left { &left.data } else { &zeros };
        for i in 0..self.data.len() {
            new_data.push(self.data[i].wrapping_add(left_datas[i]));
        };

        Color::new(new_data)
    }

    pub fn apply_up(&self,up:Option<&Color>) -> Color {
        let mut new_data = Vec::with_capacity(self.data.len());

        let zeros = vec![0;self.data.len()];
        let up_datas = if let Some(up) = up { &up.data } else { &zeros };

        for i in 0..self.data.len() {
            new_data.push(self.data[i].wrapping_add(up_datas[i]));
        };

        Color::new(new_data)
    }

    pub fn apply_avg(&self,left:Option<&Color>,up:Option<&Color>) -> Color {
        let zeros = vec![0;self.data.len()];
        let left_datas = if let Some(left) = left { &left.data } else { &zeros };
        let up_datas = if let Some(up) = up { &up.data } else { &zeros };
        let mut new_data = Vec::with_capacity(self.data.len());

        for i in 0..self.data.len() {
            let avg = (((left_datas[i] as u16) + (up_datas[i] as u16)) / 2) as u8;
            new_data.push(self.data[i].wrapping_add(avg));
        };

        Color::new(new_data)
    }

    pub fn apply_paeth(&self,left:Option<&Color>,up:Option<&Color>,left_up:Option<&Color>) -> Color {
        let zeros = vec![0;self.data.len()];
        let left_datas = if let Some(left) = left { &left.data } else { &zeros };
        let up_datas = if let Some(up) = up { &up.data } else { &zeros };
        let left_up_datas = if let Some(left_up) = left_up { &left_up.data } else { &zeros };
        let mut new_data = Vec::with_capacity(self.data.len());

        for i in 0..self.data.len() {
            let a = left_datas[i];
            let b = up_datas[i];
            let c = left_up_datas[i];
            new_data.push(self.data[i].wrapping_add(peath_predictor(a, b, c)));
        };

        Color::new(new_data)
    }
}


fn peath_predictor(a:u8,b:u8,c:u8) -> u8 {
    let a = a as isize;
    let b = b as isize;
    let c = c as isize;

    let p = a + b - c;
    let pa = ( p - a ).abs();
    let pb = ( p - b ).abs();
    let pc = ( p - c ).abs();

    if pa <= pb && pa <= pc {
        a as u8
    }else if pb <= pc {
        b as u8
    }else{
        c as u8
    }
}