use crate::geo::Point;
use crate::hrap::hrap_to_latlon;

const XOR: usize = 0;
const YOR: usize = 1;
const COLUMNS: usize = 2;
const ROWS: usize = 3;


pub struct CoordinateGenerator {
    start_x: i32,
    current_x: i32,
    current_y: i32,
    x_end: i32,
    y_end: i32
}

impl Iterator for CoordinateGenerator {
    type Item = Point;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.current_x += 1;
        if self.current_x == self.x_end {
            self.current_x = self.start_x;
            self.current_y += 1;
        }

        if self.current_y != self.y_end {
            Some(hrap_to_latlon(f64::from(self.current_x), f64::from(self.current_y)))
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Header {
    xor: i32,
    yor: i32,
    columns: i32,
    rows: i32
}

impl Header {
    pub fn from_vec(vec: Vec<i32>) -> Self {
        Self {
            xor: vec[XOR],
            yor: vec[YOR],
            columns: vec[COLUMNS],
            rows: vec[ROWS]
        }
    }

    // pub fn generate_coordinates(&self) -> Vec<Vec<Point>> {
    //     (self.yor..self.rows).map(|y| {
    //         (self.xor..self.columns).map(|x| {
    //             hrap_to_latlon(f64::from(x), f64::from(y))
    //         })
    //         .collect()
    //     })
    //     .collect()
    // }
}

impl IntoIterator for Header {
    type Item = Point;
    type IntoIter = CoordinateGenerator;

    fn into_iter(self) -> Self::IntoIter {
        CoordinateGenerator {
            start_x: self.xor,
            current_x: self.xor - 1,
            current_y: self.yor,
            x_end: self.xor + self.columns,
            y_end: self.yor + self.rows
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct DateSegments {
    month: i32,
    day: i32,
    year: i32,
    hour: i32 // in 24 hour time
}

impl DateSegments {
    // look into better strategy than indexing
    pub fn from_chars(chars: &str) -> Self {
        // assert_eq!(chars.len(), 10);
        DateSegments {
            month: chars[0..2].parse::<i32>().unwrap_or_default(),
            day: chars[2..4].parse::<i32>().unwrap_or_default(),
            year: chars[4..8].parse::<i32>().unwrap_or_default(),
            hour: chars[8..10].parse::<i32>().unwrap_or_default(),
        }
    }
}

pub fn read_old_xmrg_date(path: &str) -> DateSegments {
    let date_chars = path.chars()
        .skip_while(|c| *c < '0' || *c > '9')
        .collect::<String>();

    DateSegments::from_chars(&date_chars)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn read_old_xmrg_date_test() {
        let path = "xmrg0506199516z.gz";

        let data_segments = read_old_xmrg_date(&path);

        assert_eq!(data_segments.month, 5);
        assert_eq!(data_segments.day, 6);       
        assert_eq!(data_segments.year, 1995);
        assert_eq!(data_segments.hour, 16);
    }

    // let fake_header = Header::from_vec(vec![367, 263, 335, 159]);

    // let mut iter = fake_header.into_iter();

    // println!("first coord: {:?}", iter.next().unwrap());
    // println!("second coord: {:?}", iter.next().unwrap());
    // println!("third coord: {:?}", iter.next().unwrap());

    // let f_h = Header::from_vec(vec![367, 263 + 158 , 335, 159]);

    // let mut iter2 = f_h.into_iter();

    // println!("2 first coord: {:?}", iter2.next().unwrap());
    // println!("2 second coord: {:?}", iter2.next().unwrap());
    // println!("2 third coord: {:?}", iter2.next().unwrap());


}
