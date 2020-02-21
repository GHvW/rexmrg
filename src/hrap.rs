use crate::geo::Point;
use std::f64::consts::PI;

// HRAP : https://www.nws.noaa.gov/oh/hrl/nwsrfs/users_manual/part2/_pdf/21hrapgrid.pdf
// positive longitude values are West, Positive latitude North
// derived from https://www.nws.noaa.gov/oh/hrl/dmip/lat_lon.txt
pub fn hrap_to_latlon(x: f64, y: f64) -> Point {
    let earthr = 6371.2;
    let stlon = 105.0;
    let raddeg = 180.0 / PI;
    let xmesh = 4.7625;
    let tlat = 60.0 / raddeg;

    let _x = x - 401.0; // >
    let _y = y - 1601.0; // >

    let rr = (_x * _x) + (_y * _y);

    let gi = (earthr * (1.0 + tlat.sin())) / xmesh;
    let _gi = gi * gi;

    let rlat = ((_gi - rr) / (_gi + rr)).asin() * raddeg;

    let mut ang = _y.atan2(_x) * raddeg;

    // let if (ang.lt.0.) ang = ang + 360.0;
    ang += if ang < 0.0 { 360.0 } else { 0.0 };

    let mut rlon = 270.0 + stlon - ang;

    // let if(rlon.lt.0.) rlon=rlon+360.0;
    rlon += if rlon < 0.0 { 360.0 } else { 0.0 };

    // let if(rlon.gt.360.0) rlon = rlon - 360.0;
    rlon -= if rlon > 360.0 { 360.0 } else { 0.0 };

    Point::new(rlon, rlat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hrap_to_latlon_test() {
        // write a better test here
        let hrap_x = 367.0;
        let hrap_y = 263.0;

        let point = hrap_to_latlon(hrap_x, hrap_y);

        assert!(point.x > 106.0 && point.x < 107.0);
        assert!(point.y > 33.0 && point.y < 34.0);
    }

    // println!("long lat is {:?}", hrap_to_latlon(367.0, 263.0));
    // println!("other lat long is {:?}", hrap_to_latlon(367.0 + 335.0 , 263.0 + 159.0));
    // println!("other lat long max x, min y is {:?}", hrap_to_latlon(367.0 + 334.0 , 263.0));
    // println!("other lat long max x, min y is {:?}", hrap_to_latlon(367.0, 263.0 + 158.0));
}
