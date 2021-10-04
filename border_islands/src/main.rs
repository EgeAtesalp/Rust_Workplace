extern crate ndarray;
use ndarray::prelude::*;
use rust_algorithms::border_island::border_island::border_island;

fn main() {
    let mut a = arr2(&[
        [1, 0, 0, 1, 0, 0],
        [1, 1, 0, 1, 0, 0],
        [0, 0, 1, 0, 1, 0],
        [0, 1, 1, 0, 0, 1],
        [0, 0, 0, 1, 0, 0],
    ]);

    println!("{:?}", a);

    border_island(a);
}
