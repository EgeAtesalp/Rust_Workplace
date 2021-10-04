pub mod border_island {

    extern crate ndarray;
    use ndarray::prelude::*;

    pub fn border_island(mut m: Array2<i32>) -> Array2<i32> {
        let shape_m = m.shape();
        let row_length = shape_m[1];
        let col_length = shape_m[0];
        let mut known_borders_islands: Vec<Coordinate> = Vec::new(); //Collect the indexes of every 1 in borders
        identify_border_islands(&m, &mut known_borders_islands, row_length, col_length);

        for i in 0..row_length {
            for j in 0..col_length {
                let point = Coordinate { row: j, col: i };
                if m[[j, i]] == 1 && !vec_contains(&known_borders_islands, &point) {
                    m[[j, i]] = 0;
                }
            }
        }

        println!("{:?}", m);
        m
    }

    fn identify_border_islands(
        m: &Array2<i32>,
        x: &mut Vec<Coordinate>,
        row_length: usize,
        col_length: usize,
    ) {
        for i in [0, row_length - 1] {
            for j in 0..col_length {
                identify_adjacent(&m, x, i, j);
            }
        }
        for i in 0..row_length {
            for j in [0, col_length - 1] {
                identify_adjacent(&m, x, i, j);
            }
        }
    }
    fn identify_adjacent(m: &Array2<i32>, x: &mut Vec<Coordinate>, col: usize, row: usize) {
        let a = Coordinate { row: row, col: col };
        if m[[row, col]] == 1 && !vec_contains(x, &a) {
            x.push(a);
            let shape_m = m.shape();
            let row_length = shape_m[1];
            let col_length = shape_m[0];
            if col + 1 < row_length {
                identify_adjacent(&m, x, col + 1, row);
            }
            if col != 0 {
                identify_adjacent(&m, x, col - 1, row);
            }
            if row + 1 < col_length {
                identify_adjacent(&m, x, col, row + 1);
            }
            if row != 0 {
                identify_adjacent(&m, x, col, row - 1);
            }
        }
    }
    struct Coordinate {
        row: usize,
        col: usize,
    }

    fn vec_contains(x: &Vec<Coordinate>, y: &Coordinate) -> bool {
        if x.iter().any(|i| i.row == y.row && i.col == y.col) {
            return true;
        }
        false
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn border_island_utest_5x6() {
            let a = arr2(&[
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
    }
}
