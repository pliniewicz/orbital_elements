extern crate nalgebra as na;
use time::Duration;
use time::macros::{datetime};
use na::{U2, U3, Dynamic, ArrayStorage, VecStorage, Matrix};
use astro::consts::{GAUSS_GRAV};
pub mod Utilities;

const TIME_UNIT: f64 = 1./GAUSS_GRAV;

fn main() {

let t1 = datetime!(2028-01-03 01:00:00.000);
let t2 = datetime!(2028-02-14 02:00:00.000);
let t3 = datetime!(2028-03-25 03:00:00.000);

let time_difference1: Duration = t3 - t2;
let time_difference2: Duration = t3 - t1;
let time_difference3: Duration = t2 - t1;

let test = Duration::days(10);
// let check = time::Duration::days(time_difference1);
let check = test / (Duration::DAY * TIME_UNIT);

// let v = na::Vector3::new(1,2,3);

println!("{}\n{}\n{}",time_difference1 / (Duration::DAY * TIME_UNIT), time_difference2/ (Duration::DAY * TIME_UNIT), time_difference3/ (Duration::DAY * TIME_UNIT));
}
