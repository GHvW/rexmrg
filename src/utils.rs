// if a data point is negative, represent as -999 (no data), if positive, divide by 100 to represent in millimeters
// data points are represented as a 100th of a milimeter. .001mm is represented as 1 in a xmrg data point, dividing by 100 gets us to .001
pub fn to_mm(data_point: i16) -> f64 {
    if data_point < 0 {
        -999.0
    } else {
        data_point as f64 / 100.0
    }
}