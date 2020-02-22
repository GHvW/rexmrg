use crate::geo::Point;
use crate::hrap::hrap_to_latlon;
// use crate::read_bytes::ReadBytes;
// use crate::endian::Endian;
// use std::io::prelude::*;
// use std::io;

const XOR: usize = 0;
const YOR: usize = 1;
const COLUMNS: usize = 2;
const ROWS: usize = 3;

pub struct CoordinateGenerator {
    start_x: i32,
    current_x: i32,
    current_y: i32,
    x_end: i32,
    y_end: i32,
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
            Some(hrap_to_latlon(
                f64::from(self.current_x),
                f64::from(self.current_y),
            ))
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
    rows: i32,
}

impl Header {
    pub fn from_vec(vec: Vec<i32>) -> Self {
        Self {
            xor: vec[XOR],
            yor: vec[YOR],
            columns: vec[COLUMNS],
            rows: vec[ROWS],
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
            y_end: self.yor + self.rows,
        }
    }
}

pub struct Build1997Header {
    user_id: String,
    saved_datetime: String,
    process_flag: String,
}

impl Build1997Header {
    pub fn new(user_id: String, saved_datetime: String, process_flag: String) -> Self {
        Build1997Header {
            user_id,
            saved_datetime,
            process_flag,
        }
    }
}

pub struct Build4_2Additions {
    valid_datetime: String,
    max_value: i32,
    version_number: f32,
}

impl Build4_2Additions {
    pub fn new(valid_datetime: String, max_value: i32, version_number: f32) -> Self {
        Build4_2Additions {
            valid_datetime,
            max_value,
            version_number,
        }
    }
}

pub struct Build4_2Header {
    original: Build1997Header,
    build_4_2_additions: Build4_2Additions,
}

impl Build4_2Header {
    pub fn new(
        user_id: String,
        saved_datetime: String,
        process_flag: String,
        valid_datetime: String,
        max_value: i32,
        version_number: f32,
    ) -> Self {
        Build4_2Header {
            original: Build1997Header::new(user_id, saved_datetime, process_flag),
            build_4_2_additions: Build4_2Additions::new(valid_datetime, max_value, version_number),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OperSys {
    HP,
    LX,
}

pub struct Build5_2_2Header {
    operating_system: OperSys,
    user_id: String,
    saved_datetime: String,
    process_flag: String,
    build_4_2_additions: Build4_2Additions,
}

impl Build5_2_2Header {
    pub fn new(
        operating_system: OperSys,
        user_id: String,
        saved_datetime: String,
        process_flag: String,
        valid_datetime: String,
        max_value: i32,
        version_number: f32,
    ) -> Self {
        Build5_2_2Header {
            operating_system,
            user_id,
            saved_datetime,
            process_flag,
            build_4_2_additions: Build4_2Additions::new(valid_datetime, max_value, version_number),
        }
    }

    // change name
    pub fn new_2(
        operating_system: OperSys,
        user_id: String,
        saved_datetime: String,
        process_flag: String,
        build_4_2_additions: Build4_2Additions) -> Self {
            Build5_2_2Header {
                operating_system,
                user_id,
                saved_datetime,
                process_flag,
                build_4_2_additions
            }
        }
}

// pub fn build_42_reader<R: Read>(reader: &mut R, endian: Endian) -> io::Result<Option<Build4_2Additions>> {

//     let valid_datetime = ReadBytes::new(20, endian).read_u8s(reader)?;
//     let max_value = endian;
//     let version_number = read_1.read_u8s(reader: &mut R)

// }

// pub fn build_522_reader<R: Read>(reader: &mut R, endian: Endian) -> io::Result<Option<Build5_2_2Header>> {
//     let op_bytes: Vec<u8> = (0..2).map(|_| endian.read_u8(reader)).collect()?;
//     let user_id: Vec<u8> = (0..8).map(|_| endian.read_u8(reader)).collect()?;
    // let saved_datetime = (0..20).map(|_| endian.read_u8(reader)).collect()?;
    // let process_flag = (0..8).map(|_| endian.read_u8(reader)).collect()?;
//     let build_4_2_additions = build_42_reader(&reader, endian)?;

//     Header5_2_2(Build5_2_2Header::new(op))
// }

pub enum Metadata {
    Header1997(Build1997Header),
    Header4_2(Build4_2Header),
    Header5_2_2(Build5_2_2Header),
}

impl Metadata {
    // DEBUG - come back to this
    pub fn datetime(&self) -> String {
        match self {
            Metadata::Header1997(header) => header.saved_datetime.clone(),
            Metadata::Header4_2(header) => header.original.saved_datetime.clone(),
            Metadata::Header5_2_2(header) => header.saved_datetime.clone(),
        }
    }

    pub fn os(&self) -> Option<OperSys> {
        match self {
            Metadata::Header1997(_) => None,
            Metadata::Header4_2(_) => None,
            Metadata::Header5_2_2(header) => Some(header.operating_system),
        }
    }

    pub fn max_value(&self) -> Option<i32> {
        match self {
            Metadata::Header1997(_) => None,
            Metadata::Header4_2(header) => Some(header.build_4_2_additions.max_value),
            Metadata::Header5_2_2(header) => Some(header.build_4_2_additions.max_value),
        }
    }

    // DEBUG - revisit this one
    pub fn user_id(&self) -> Option<String> {
        match self {
            Metadata::Header1997(header) => Some(header.user_id.clone()),
            Metadata::Header4_2(header) => Some(header.original.user_id.clone()),
            Metadata::Header5_2_2(header) => Some(header.user_id.clone()),
        }
    }

    pub fn process_flag(&self) -> Option<String> {
        match self {
            Metadata::Header1997(header) => Some(header.process_flag.clone()),
            Metadata::Header4_2(header) => Some(header.original.process_flag.clone()),
            Metadata::Header5_2_2(header) => Some(header.process_flag.clone()),
        }
    }

    pub fn version(&self) -> Option<f32> {
        match self {
            Metadata::Header1997(_) => None,
            Metadata::Header4_2(header) => Some(header.build_4_2_additions.version_number),
            Metadata::Header5_2_2(header) => Some(header.build_4_2_additions.version_number),
        }
    }

    pub fn valid_datetime(&self) -> Option<String> {
        match self {
            Metadata::Header1997(_) => None,
            Metadata::Header4_2(header) => Some(header.build_4_2_additions.valid_datetime.clone()),
            Metadata::Header5_2_2(header) => {
                Some(header.build_4_2_additions.valid_datetime.clone())
            }
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
}
