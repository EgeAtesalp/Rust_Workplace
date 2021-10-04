extern crate ndarray;
use ndarray::prelude::*;
use rust_algorithms::border_island::border_island::border_island;

#[test]
fn border_island_itest_5x6() {
    let mut a = arr2(&[
        [1, 0, 0, 1, 0, 0],
        [1, 1, 0, 1, 0, 0],
        [0, 0, 1, 0, 1, 0],
        [0, 1, 1, 0, 0, 1],
        [0, 0, 0, 1, 0, 0],
    ]);
    let b = arr2(&[
        [1, 0, 0, 1, 0, 0],
        [1, 1, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1],
        [0, 0, 0, 1, 0, 0],
    ]);
    assert_eq!(border_island(a), b);
}
