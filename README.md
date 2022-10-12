# Orbital elements in Rust

It's a project for my celestial mechanics class in fall 2022.
I decided to upload it here for storage/future use.
Uses Gauss's method of orbit determination.

## By far:
What it does right now is prints Sun's rectangular geocentric coordinates for 3 dates of observation (for now they are set, working on I/O).
Now is also capable of handling RADEC direction cosine vector for.

## Dependencies
* astro for astronomical utilities, like Julian days
* nalgebra for vector handling

## Compilation

I recommend using *cargo* manager:
```
git clone https://github.com/pliniewicz/orbital_elements.git .
cd orbital_elements
cargo build
cargo run
```
where *cargo build* can be ommited and one could *run* immediately.

## Plans
* use both Gauss and Laplace methods & choose between them
* handle arbitrary input
* draw orbit once elements are determined, eventually
