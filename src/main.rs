// temporary warning silencer
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
extern crate nalgebra as na;
extern crate astro;
use na::{vector};
use astro::consts::{GAUSS_GRAV};
use astro::*;

const TIME_UNIT: f64 = 1./GAUSS_GRAV;

fn main() {

// wkładam dane chwil czasu
let data1 = time::Date{year: 2028, month: 1, decimal_day: time::decimal_day(&time::DayOfMonth{day: 3, hr: 1, min: 0, sec: 0.0, time_zone: 0.0}), cal_type: time::CalType::Gregorian};
let data2 = time::Date{year: 2028, month: 2, decimal_day: time::decimal_day(&time::DayOfMonth{day: 14, hr: 2, min: 0, sec: 0.0, time_zone: 0.0}), cal_type: time::CalType::Gregorian};
let data3 = time::Date{year: 2028, month: 3, decimal_day: time::decimal_day(&time::DayOfMonth{day: 25, hr: 3, min: 0, sec: 0.0, time_zone: 0.0}), cal_type: time::CalType::Gregorian};

// wkładam współrzędne
let alpha1 = angle::deg_frm_hms(11, 42, 21.2960).to_radians();
let alpha2 = angle::deg_frm_hms(11, 55, 35.4617).to_radians();
let alpha3 = angle::deg_frm_hms(11, 32, 10.1277).to_radians();
let delta1 = angle::deg_frm_dms(-14, 29, 39.806).to_radians();
let delta2 = angle::deg_frm_dms(-7, 36, 14.928).to_radians();
let delta3 = angle::deg_frm_dms(7, 51, 20.425).to_radians();

// obliczam różnice dni juliańskich
let time_difference1 = ( time::julian_day(&data3)-time::julian_day(&data2) ) / TIME_UNIT;
let time_difference2 = ( time::julian_day(&data3)-time::julian_day(&data1) ) / TIME_UNIT;
let time_difference3 = ( time::julian_day(&data2)-time::julian_day(&data1) ) / TIME_UNIT;

// wyznaczam współrzędne geocentryczne słońca w 3 chwilach czasu
let (point1, dist1) = sun::geocent_ecl_pos(time::julian_day(&data1));
let (X1,Y1,Z1) = sun::geocent_rect_coords(point1.lat, point1.long, dist1, ecliptic::mn_oblq_laskar(time::julian_day(&data1)));
let (point2, dist2) = sun::geocent_ecl_pos(time::julian_day(&data2));
let (X2,Y2,Z2) = sun::geocent_rect_coords(point2.lat, point2.long, dist2, ecliptic::mn_oblq_laskar(time::julian_day(&data2)));
let (point3, dist3) = sun::geocent_ecl_pos(time::julian_day(&data3));
let (X3,Y3,Z3) = sun::geocent_rect_coords(point3.lat, point3.long, dist3, ecliptic::mn_oblq_laskar(time::julian_day(&data3)));

// wektory obserwacyjne
let rho1 = na::vector![alpha1.cos()*delta1.cos(),alpha1.sin()*delta1.cos(),delta1.sin()];
let rho2 = na::vector![alpha2.cos()*delta2.cos(),alpha2.sin()*delta2.cos(),delta2.sin()];
let rho3 = na::vector![alpha3.cos()*delta3.cos(),alpha3.sin()*delta3.cos(),delta3.sin()];

// iloczyny wektorowe
let p1 = rho2.cross(&rho3);
let p2 = rho1.cross(&rho3);
let p3 = rho1.cross(&rho2);

// println!("{}, {}, {}", rho1, rho2, rho3);
// println!("t3-t1: {}", time_difference1);
// println!("t3-t2: {}", time_difference2);
// println!("t2-t1: {}", time_difference3);
// println!("X1 :{} au\nY1: {} au\nZ1: {} au" ,X1, Y1, Z1);
// println!("X2 :{} au\nY2: {} au\nZ2: {} au" ,X2, Y2, Z2);
// println!("X3 :{} au\nY3: {} au\nZ3: {} au" ,X3, Y3, Z3);
}
