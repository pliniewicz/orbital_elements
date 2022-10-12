// temporary warning silencer
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
extern crate nalgebra as na;
extern crate astro;
use na::{U2, U3, Dynamic, ArrayStorage, VecStorage, Matrix};
use astro::consts::{GAUSS_GRAV};
use astro::*;

const TIME_UNIT: f64 = 1./GAUSS_GRAV;

fn main() {

// wkładam dane chwil czasu
let data1 = time::Date{year: 2028, month: 1, decimal_day: time::decimal_day(&time::DayOfMonth{day: 3, hr: 1, min: 0, sec: 0.0, time_zone: 0.0}), cal_type: time::CalType::Gregorian};
let data2 = time::Date{year: 2028, month: 2, decimal_day: time::decimal_day(&time::DayOfMonth{day: 14, hr: 2, min: 0, sec: 0.0, time_zone: 0.0}), cal_type: time::CalType::Gregorian};
let data3 = time::Date{year: 2028, month: 3, decimal_day: time::decimal_day(&time::DayOfMonth{day: 25, hr: 3, min: 0, sec: 0.0, time_zone: 0.0}), cal_type: time::CalType::Gregorian};

// obliczam różnice dni juliańskich
let time_difference1 = time::julian_day(&data3)-time::julian_day(&data2);
let time_difference2 = time::julian_day(&data3)-time::julian_day(&data1);
let time_difference3 = time::julian_day(&data2)-time::julian_day(&data1);

// wyznaczam współrzędne geocentryczne słońca w 3 chwilach czasu
let (point1, dist1) = sun::geocent_ecl_pos(time::julian_day(&data1));
let (X1,Y1,Z1) = sun::geocent_rect_coords(point1.lat, point1.long, dist1, ecliptic::mn_oblq_laskar(time::julian_day(&data1)));
let (point2, dist2) = sun::geocent_ecl_pos(time::julian_day(&data2));
let (X2,Y2,Z2) = sun::geocent_rect_coords(point2.lat, point2.long, dist2, ecliptic::mn_oblq_laskar(time::julian_day(&data2)));
let (point3, dist3) = sun::geocent_ecl_pos(time::julian_day(&data3));
let (X3,Y3,Z3) = sun::geocent_rect_coords(point3.lat, point3.long, dist3, ecliptic::mn_oblq_laskar(time::julian_day(&data3)));

// println!("{}", data1);
println!("X1 :{} au\nY1: {} au\nZ1: {} au" ,X1, Y1, Z1);
println!("X2 :{} au\nY2: {} au\nZ2: {} au" ,X2, Y2, Z2);
println!("X3 :{} au\nY3: {} au\nZ3: {} au" ,X3, Y3, Z3);
}
