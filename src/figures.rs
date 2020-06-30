pub mod figures {
    extern crate rand;
    use rand::prelude::*;
    use crate::playfield::Coords;

    #[derive(Clone, PartialEq, Debug)]
    pub enum Shape {
        NoShape,
        OShape,
        IShape,
        TShape,
        JShape,
        LShape,
        SShape,
        ZShape,
    }

    impl Default for Shape {
        fn default() -> Self { Shape::NoShape }
    }

    pub const LAYOUT_WIDTH: i8 = 4;
    pub const LAYOUT_HEIGHT: i8 = 4;
    pub type Layout = [[u8; LAYOUT_WIDTH as usize]; LAYOUT_HEIGHT as usize];

    #[derive(Clone)]
    pub struct Tetromino {
        pub shape: Shape,
        layout: Layout,
        iter_row: i8,
        iter_col: i8,
    }

    fn super_rotation_i(layout: &mut Layout) {
        /* rotate following edge items
         *   0123
         * 0 --*-
         * 1 *---
         * 2 ---*
         * 3 -*--
         */
        let temp = layout[0][2];
        layout[0][2] = layout[1][0];
        layout[1][0] = layout[3][1];
        layout[3][1] = layout[2][3];
        layout[2][3] = temp;
        /* rotate following edge items
         *   0123
         * 0 -*--
         * 1 ---*
         * 2 *---
         * 3 --*-
         */
        let temp = layout[0][1];
        layout[0][1] = layout[2][0];
        layout[2][0] = layout[3][2];
        layout[3][2] = layout[1][3];
        layout[1][3] = temp;
        /* rotate inner pieces
         *   0123
         * 0 ----
         * 1 -**-
         * 2 -**-
         * 3 ----
         */
        let temp = layout[1][1];
        layout[1][1] = layout[2][1];
        layout[2][1] = layout[2][2];
        layout[2][2] = layout[1][2];
        layout[1][2] = temp;
    }

    fn rotate_special_i(layout: &mut Layout) {
        /* simply transpose matrix */
        for row in 0..4 {
            for col in row..4 {
                let temp = layout[row][col];
                layout[row][col] = layout[col][row];
                layout[col][row] = temp;
            }
        }
    }

    fn rotate_special_s(layout: &mut Layout) {
        /* s shape switches between two states */
        let original_layout = Tetromino::new(Shape::SShape).layout;
        let turned_layout = [[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]];
        if *layout == original_layout {
            *layout = turned_layout;
        } else {
            *layout = original_layout;
        }
    }

    fn rotate_special_z(layout: &mut Layout) {
        /* z shape switches between two states */
        let original_layout = Tetromino::new(Shape::ZShape).layout;
        let turned_layout = [[0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
        if *layout == original_layout {
            *layout = turned_layout;
        } else {
            *layout = original_layout;
        }
    }

    fn rotate_ordinary(layout: &mut Layout) {
        /* rotate corner pieces clockwise
         *   0123
         * 0 *-*-
         * 1 ----
         * 2 *-*-
         * 3 ----
         */
        let temp = layout[0][2];
        layout[0][2] = layout[0][0];
        layout[0][0] = layout[2][0];
        layout[2][0] = layout[2][2];
        layout[2][2] = temp;
        /* rotate in-between corner pieces clockwise
         *   0123
         * 0 -*--
         * 1 *-*-
         * 2 -*--
         * 3 ----
         */
        let temp = layout[0][1];
        layout[0][1] = layout[1][0];
        layout[1][0] = layout[2][1];
        layout[2][1] = layout[1][2];
        layout[1][2] = temp;
    }

    pub fn rotate(tetromino: &mut Tetromino) {
        match tetromino.shape {
            /* o doesn't need any rotation */
            Shape::OShape => (),
            /* i, s, z are not "just rotations" */
            Shape::IShape => rotate_special_i(&mut tetromino.layout),
            Shape::SShape => rotate_special_s(&mut tetromino.layout),
            Shape::ZShape => rotate_special_z(&mut tetromino.layout),
            _ => rotate_ordinary(&mut tetromino.layout),
        }
    }

    impl Tetromino {
        pub fn new(shape: Shape) -> Tetromino {
            let layout = match shape {
                Shape::OShape => [[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
                Shape::IShape => [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
                Shape::TShape => [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                Shape::JShape => [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                Shape::LShape => [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                Shape::SShape => [[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
                Shape::ZShape => [[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]],
                Shape::NoShape => [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            };
            Tetromino {shape: shape, layout: layout, iter_row: 0, iter_col: 0}
        }

        pub fn new_random() -> Tetromino {
            let mut rng = thread_rng();
            let shape = match rng.gen_range(0, 7) {
                0 => Shape::OShape,
                1 => Shape::IShape,
                2 => Shape::TShape,
                3 => Shape::JShape,
                4 => Shape::LShape,
                5 => Shape::SShape,
                _ => Shape::ZShape,
            };
            Tetromino::new(shape)
        }

        pub fn shape_at(self: &Self, coords: &Coords) -> Shape {
            match self.layout[coords.row as usize][coords.col as usize] {
                0 => Shape::NoShape,
                _ => self.shape.clone(),
            }
        }
    }

    impl Iterator for Tetromino {
        type Item = (Coords, Shape);

        fn next(&mut self) -> Option<Self::Item> {
            let coords = Coords{row: self.iter_row, col: self.iter_col};
            self.iter_col += 1;
            if self.iter_col == LAYOUT_WIDTH {
                self.iter_col = 0;
                self.iter_row += 1;
            }
            if self.iter_row == LAYOUT_HEIGHT {
                self.iter_row = 0;
                self.iter_col = 0;
                return None;
            }

            Some((coords, self.shape_at(&coords)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::figures::*;
    use crate::playfield::Coords;

    fn assert_shape(tetro: &Tetromino, binary_array: &Layout) {
        for row in 0..LAYOUT_HEIGHT {
            for col in 0..LAYOUT_WIDTH {
                let coords = Coords{row: row, col: col};
                if binary_array[row as usize][col as usize] == 1 {
                    assert_eq!(tetro.shape_at(&coords), tetro.shape, "(row: {} col: {})", row, col);
                } else {
                    assert_eq!(tetro.shape_at(&coords), Shape::NoShape, "(row: {} col: {})", row, col);
                }
            }
        }
    }

    #[test]
    fn create_o_shape() {
        let f = Tetromino::new(Shape::OShape);
        assert_eq!(f.shape, Shape::OShape);
        assert_shape(&f, &[[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]);
    }

    #[test]
    fn rotate_o_shape() {
        let mut f = Tetromino::new(Shape::OShape);
        /* o shape remains the same, no matter how much you rotate it */
        /* 90 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]);
        /* 180 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]);
        /* 270 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]);
        /* 360 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [0, 1, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]]);
    }

     #[test]
    fn rotate_i_shape() {
        let mut f = Tetromino::new(Shape::IShape);
        /* i shape switches between two states */
        /* 90 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]]);
        /* 180 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]]);
        /* 270 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]]);
        /* 360 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]]);
    }

    #[test]
    fn rotate_t_shape() {
        let mut f = Tetromino::new(Shape::TShape);
        /* 90 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 1, 0, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]]);
        /* 180 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]]);
        /* 270 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]]);
        /* 360 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    }

    #[test]
    fn rotate_j_shape() {
        let mut f = Tetromino::new(Shape::JShape);
        /* 90 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 1, 1, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]]);
        /* 180 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [1, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]]);
        /* 270 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]]);
        /* 360 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    }

    #[test]
    fn rotate_l_shape() {
        let mut f = Tetromino::new(Shape::LShape);
        /* 90 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]);
        /* 180 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [1, 1, 1, 0], [1, 0, 0, 0], [0, 0, 0, 0]]);
        /* 270 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]]);
        /* 360 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]]);
    }

    #[test]
    fn rotate_s_shape() {
        let mut f = Tetromino::new(Shape::SShape);
        /* 90 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]]);
        /* 180 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]]);
        /* 270 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]]);
        /* 360 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0]]);
    }

    #[test]
    fn rotate_z_shape() {
        let mut f = Tetromino::new(Shape::ZShape);
        /* 90 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]]);
        /* 180 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]);
        /* 270 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 1, 0], [0, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0]]);
        /* 360 degrees */
        rotate(&mut f);
        assert_shape(&f, &[[0, 0, 0, 0], [1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]);
    }

    #[test]
    fn iterator_interface() {
        let f = Tetromino::new(Shape::ZShape);
        let result = [[Shape::NoShape, Shape::NoShape, Shape::NoShape, Shape::NoShape],
                      [Shape::ZShape, Shape::ZShape, Shape::NoShape, Shape::NoShape],
                      [Shape::NoShape, Shape::ZShape, Shape::ZShape, Shape::NoShape],
                      [Shape::NoShape, Shape::NoShape, Shape::NoShape, Shape::NoShape]];
        for (coords, shape) in f {
            assert_eq!(shape, result[coords.row as usize][coords.col as usize]);
        }
    }
}
