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


pub struct Build1997Header {
    user_id: String,
    saved_datetime: String,
    process_flag: String
}

pub struct Build4_2Additions {
    valid_datetime: String,
    max_value: i32,
    version_number: f32
}

pub struct Build4_2Header {
    original: Build1997Header,
    build_4_2_additions: Build4_2Additions 
}

#[derive(Debug, Copy, Clone)]
pub enum OperSys {
    HP,
    LX
}

pub struct Build5_2_2Header {
    operating_system: OperSys,
    user_id: String,
    saved_datetime: String,
    process_flag: String,
    build_4_2_additions: Build4_2Additions
}

pub enum MetaData {
    Header1997(Build1997Header),
    Header4_2(Build4_2Header),
    Header5_2_2(Build5_2_2Header)
}

impl MetaData {
    // DEBUG - come back to this
    pub fn datetime(&self) -> String {
        match self {
            MetaData::Header1997(header) => header.saved_datetime.clone(),
            MetaData::Header4_2(header) => header.original.saved_datetime.clone(),
            MetaData::Header5_2_2(header) => header.saved_datetime.clone()
        }
    }

    pub fn os(&self) -> Option<OperSys> {
        match self {
            MetaData::Header1997(_) => None,
            MetaData::Header4_2(_) => None,
            MetaData::Header5_2_2(header) => Some(header.operating_system)
        }
    }

    pub fn max_value(&self) -> Option<i32> {
        match self {
            MetaData::Header1997(_) => None,
            MetaData::Header4_2(header) => Some(header.build_4_2_additions.max_value),
            MetaData::Header5_2_2(header) => Some(header.build_4_2_additions.max_value)
        }
    }

    // DEBUG - revisit this one
    pub fn user_id(&self) -> Option<String> {
        match self {
            MetaData::Header1997(header) => Some(header.user_id.clone()),
            MetaData::Header4_2(header) => Some(header.original.user_id.clone()),
            MetaData::Header5_2_2(header) => Some(header.user_id.clone())
        }
    }

    pub fn process_flag(&self) -> Option<String> {
        match self {
            MetaData::Header1997(header) => Some(header.process_flag.clone()),
            MetaData::Header4_2(header) => Some(header.original.process_flag.clone()),
            MetaData::Header5_2_2(header) => Some(header.process_flag.clone())
        }
    }

    pub fn version(&self) -> Option<f32> {
        match self {
            MetaData::Header1997(_) => None,
            MetaData::Header4_2(header) => Some(header.build_4_2_additions.version_number),
            MetaData::Header5_2_2(header) => Some(header.build_4_2_additions.version_number)
        }
    }

    pub fn valid_datetime(&self) -> Option<String> {
        match self {
            MetaData::Header1997(_) => None,
            MetaData::Header4_2(header) => Some(header.build_4_2_additions.valid_datetime.clone()),
            MetaData::Header5_2_2(header) => Some(header.build_4_2_additions.valid_datetime.clone())
        }
    }
}

// #[derive(Debug, Copy, Clone)]
// pub struct DateSegments {
//     month: i32,
//     day: i32,
//     year: i32,
//     hour: i32 // in 24 hour time
// }

// impl DateSegments {
//     // look into better strategy than indexing
//     pub fn from_chars(chars: &str) -> Self {
//         // assert_eq!(chars.len(), 10);
//         DateSegments {
//             month: chars[0..2].parse::<i32>().unwrap_or_default(),
//             day: chars[2..4].parse::<i32>().unwrap_or_default(),
//             year: chars[4..8].parse::<i32>().unwrap_or_default(),
//             hour: chars[8..10].parse::<i32>().unwrap_or_default(),
//         }
//     }

//     pub fn str_from_date(&self) -> String {
//         format!("{}-{}-{} {}:00:00", self.month, self.day, self.month, self.hour)
//     }
// }

// pub fn read_xmrg_date(path: &str) -> Option<DateSegments> {
//     // let date_chars = path.chars().enumerate()
//     //     // .skip_while(|c| *c < '0' || *c > '9')
//     //     .skip_while(|c| *c != '_')
//     //     .collect::<String>();
//     let mut started = false;
//     let mut count = 0;
//     let mut start = 0;
//     for (i, c) in path.chars().enumerate() {
//         if c >= '0' && c <= '9' {
//             if !started {
//                 start = i;
//                 started = true;
//             }
//             count += 1;
//         } else {
//             if started {
//                 started = false;
//                 count = 0;
//             }
//         }

//         if count == 8 {
//             return Some(DateSegments::from_chars(&path[start..]))
//         }
//     }

//     // DateSegments::from_chars(&date_chars)
//     None
// }



#[cfg(test)]
mod tests {
    // use super::*;


    // #[test]
    // fn read_file_name_date_test() {
    //     let path = "xmrg0506199516z.gz";

    //     let data_segments = read_xmrg_date(&path).unwrap();

    //     assert_eq!(data_segments.month, 5);
    //     assert_eq!(data_segments.day, 6);       
    //     assert_eq!(data_segments.year, 1995);
    //     assert_eq!(data_segments.hour, 16);

    //     let p2 = "xmrg6_2020021600f006.gz";

    //     let ds2 = read_xmrg_date(&p2).unwrap();
    // }

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
