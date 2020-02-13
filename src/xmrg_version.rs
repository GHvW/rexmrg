
// see the second record section of https://www.nws.noaa.gov/oh/hrl/misc/xmrg.pdf
#[derive(Debug, Eq, PartialEq)]
pub enum XmrgVersion {
    Pre1997,
    Build4_2,
    Build5_2_2,
}


pub fn get_xmrg_version(byte_count: i32, max_x: i32) -> Option<XmrgVersion> {
    match byte_count {
        66 => Some(XmrgVersion::Build5_2_2),
        38 => Some(XmrgVersion::Build4_2), // a 37 byte version may be valid. Consider adding
        n if n == max_x * 2 => Some(XmrgVersion::Pre1997),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn get_xmrg_version_test() {
        let columns = 100;
        let first_byte_count = 66;
        let second_byte_count = 38;

        let v1 = get_xmrg_version(first_byte_count, columns);
        assert_eq!(v1, Some(XmrgVersion::Build5_2_2));

        let v2 = get_xmrg_version(second_byte_count, columns);
        assert_eq!(v2, Some(XmrgVersion::Build4_2));

        let v3 = get_xmrg_version(columns * 2, columns);
        assert_eq!(v3, Some(XmrgVersion::Pre1997));
    }
}
