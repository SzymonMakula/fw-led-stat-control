use std::collections::HashMap;

use crate::matrix::{EMPTY_MATRIX, Matrix, MATRIX_WIDTH};
use crate::picture::Picture;

struct Canvas {
    painters: HashMap<String, Painter>,
}

struct Painter {
    offset_x: usize,
    offset_y: usize,
    picture_height: usize,
    picture_width: usize,
    painter: Box<dyn Picture>,
}

impl Painter {
    // Returns space taken by Picture as Matrix. non 0 values indicate space taken.
    fn get_space_as_matrix(&self) -> Matrix {
        let mut output = Vec::from(EMPTY_MATRIX);

        for row in 0..self.picture_height {
            for col in 0..self.picture_width {
                output[row * MATRIX_WIDTH + col] = 1
            }
        }
        let matrix = Matrix::try_from(output.as_slice())
            .expect("Input vector should be sanitized to match required schema");

        matrix.shift_matrix(self.offset_x, self.offset_y)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum AddPainterError {
    SpaceTaken,
}
impl Canvas {
    // Get Matrix of all vacant/busy slots
    fn get_space_matrix(&self) -> Matrix {
        self.painters
            .iter()
            .map(|(_, painter)| painter.get_space_as_matrix())
            .reduce(|mut acc, e| {
                acc.join_matrix(&e);
                acc
            })
            .unwrap_or(Matrix::default())
    }

    // Add Painter to registered Painters
    fn add_painter(
        &mut self,
        painter_key: String,
        painter: Painter,
    ) -> Result<(), AddPainterError> {
        let is_vacant = self.is_space_vacant(&painter);

        if !is_vacant {
            return Err(AddPainterError::SpaceTaken);
        }

        self.painters.insert(painter_key, painter);
        Ok(())
    }

    // Call .draw() for all Painters and return the resulting Matrix
    fn paint_matrix(&mut self) -> Matrix {
        self.painters
            .iter_mut()
            .map(|(_, painter)| {
                painter
                    .painter
                    .draw()
                    .shift_matrix(painter.offset_x, painter.offset_y)
            })
            .reduce(|mut acc, e| {
                acc.join_matrix(&e);
                acc
            })
            .unwrap_or(Matrix::default())
    }

    // Check if Painter has enough space to paint its picture
    fn is_space_vacant(&self, painter: &Painter) -> bool {
        let space_matrix = self.get_space_matrix();
        for row in 0..painter.picture_height {
            for col in 0..painter.picture_width {
                if space_matrix.get_el(row + painter.offset_y, col + painter.offset_x) > 0 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod painter_tests {
    use crate::canvas::Painter;
    use crate::matrix::Matrix;
    use crate::picture::Picture;

    struct PainterMock {}

    impl Picture for PainterMock {
        // The implementation does not matter for the tests
        fn draw(&mut self) -> Matrix {
            Matrix::default()
        }
    }

    #[test]
    fn gets_space_owned_by_3_by_5_matrix() {
        let painter = Painter {
            offset_x: 2,
            offset_y: 2,
            picture_height: 3,
            picture_width: 5,
            painter: Box::new(PainterMock {}),
        };

        #[rustfmt::skip]
        let expected_buffer =
               [0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 1, 1, 1, 1, 0, 0,
                0, 0, 1, 1, 1, 1, 1, 0, 0,
                0, 0, 1, 1, 1, 1, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];

        let expected = Matrix::try_from(expected_buffer.as_slice()).unwrap();

        let actual = painter.get_space_as_matrix();

        assert_eq!(expected, actual)
    }
}

#[cfg(test)]
mod canvas_tests {
    use std::collections::HashMap;

    use crate::canvas::{AddPainterError, Canvas, Painter};
    use crate::matrix::Matrix;
    use crate::picture::Picture;

    struct PainterMock {}

    impl Picture for PainterMock {
        // The implementation does not matter for the tests
        fn draw(&mut self) -> Matrix {
            Matrix::default()
        }
    }

    #[test]
    fn get_matrix_with_no_pictures() {
        let canvas = Canvas {
            painters: HashMap::new(),
        };

        let expected_matrix = Matrix::default();

        let actual_matrix = canvas.get_space_matrix();

        assert_eq!(expected_matrix, actual_matrix)
    }

    #[test]
    fn get_matrix_with_3_pictures() {
        let painter_1 = Painter {
            picture_width: 2,
            picture_height: 2,
            offset_x: 0,
            offset_y: 0,
            painter: Box::new(PainterMock {}),
        };
        let painter_2 = Painter {
            picture_width: 2,
            picture_height: 2,
            offset_x: 2,
            offset_y: 2,
            painter: Box::new(PainterMock {}),
        };
        let painter_3 = Painter {
            picture_width: 2,
            picture_height: 2,
            offset_x: 4,
            offset_y: 4,
            painter: Box::new(PainterMock {}),
        };

        let canvas = Canvas {
            painters: HashMap::from([
                ("painter_one".to_string(), painter_1),
                ("painter_two".to_string(), painter_2),
                ("painter_three".to_string(), painter_3),
            ]),
        };
        #[rustfmt::skip]
        let expected_buffer =
               [1, 1, 0, 0, 0, 0, 0, 0, 0,
                1, 1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 1, 0, 0, 0, 0, 0,
                0, 0, 1, 1, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 1, 1, 0, 0, 0,
                0, 0, 0, 0, 1, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];

        let expected_matrix = Matrix::try_from(expected_buffer.as_slice()).unwrap();

        let actual_matrix = canvas.get_space_matrix();

        assert_eq!(expected_matrix, actual_matrix)
    }

    #[test]
    fn is_space_vacant_when_vacant() {
        let painter_1 = Painter {
            picture_width: 2,
            picture_height: 2,
            offset_x: 0,
            offset_y: 0,
            painter: Box::new(PainterMock {}),
        };
        let painter_2 = Painter {
            picture_width: 2,
            picture_height: 2,
            offset_x: 2,
            offset_y: 0,
            painter: Box::new(PainterMock {}),
        };
        let canvas = Canvas {
            painters: HashMap::from([("painter_one".to_string(), painter_1)]),
        };
        assert_eq!(canvas.is_space_vacant(&painter_2), true)
    }

    #[test]
    fn is_space_vacant_when_busy() {
        let painter_1 = Painter {
            picture_width: 2,
            picture_height: 2,
            offset_x: 0,
            offset_y: 0,
            painter: Box::new(PainterMock {}),
        };
        let painter_2 = Painter {
            picture_width: 2,
            picture_height: 2,
            offset_x: 1, // Intersects by two points
            offset_y: 0,
            painter: Box::new(PainterMock {}),
        };
        let canvas = Canvas {
            painters: HashMap::from([("painter_one".to_string(), painter_1)]),
        };
        assert_eq!(canvas.is_space_vacant(&painter_2), false)
    }

    #[test]
    fn paint_matrix_with_2_painters() {
        struct Painter1 {}
        impl Picture for Painter1 {
            fn draw(&mut self) -> Matrix {
                #[rustfmt::skip]
                let picture =
                       [1, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 1, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ];
                Matrix::try_from(picture.as_slice()).unwrap()
            }
        }

        struct Painter2 {}
        impl Picture for Painter2 {
            fn draw(&mut self) -> Matrix {
                #[rustfmt::skip]
                let picture =
                       [0, 1, 0, 0, 0, 0, 0, 0, 0,
                        1, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ];
                Matrix::try_from(picture.as_slice()).unwrap()
            }
        }

        let painter_1 = Painter {
            picture_width: 2,
            picture_height: 2,
            offset_x: 0,
            offset_y: 0,
            painter: Box::new(Painter1 {}),
        };
        let painter_2 = Painter {
            picture_width: 2,
            picture_height: 2,
            offset_x: 2,
            offset_y: 0,
            painter: Box::new(Painter2 {}),
        };

        let mut canvas = Canvas {
            painters: HashMap::from([
                ("painter_one".to_string(), painter_1),
                ("painter_two".to_string(), painter_2),
            ]),
        };

        #[rustfmt::skip]
        let expected_buffer =
               [1, 0, 0, 1, 0, 0, 0, 0, 0,
                0, 1, 1, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];

        let expected_matrix = Matrix::try_from(expected_buffer.as_slice()).unwrap();

        assert_eq!(canvas.paint_matrix(), expected_matrix);
    }

    #[test]
    fn adds_painter_resolves_when_vacant() {
        let painter = Painter {
            picture_width: 2,
            picture_height: 2,
            offset_x: 0,
            offset_y: 0,
            painter: Box::new(PainterMock {}),
        };

        let mut canvas = Canvas {
            painters: HashMap::from([]),
        };

        assert_eq!(canvas.painters.is_empty(), true);

        canvas.add_painter("painter".to_string(), painter).unwrap();

        assert_eq!(canvas.painters.len(), 1)
    }

    #[test]
    fn add_painter_rejects_when_space_taken() {
        let painter_1 = Painter {
            picture_width: 2,
            picture_height: 2,
            offset_x: 0,
            offset_y: 0,
            painter: Box::new(PainterMock {}),
        };
        // Intersects with painter_1 boundaries
        let painter_2 = Painter {
            picture_width: 4,
            picture_height: 4,
            offset_x: 1,
            offset_y: 1,
            painter: Box::new(PainterMock {}),
        };

        let mut canvas = Canvas {
            painters: HashMap::from([]),
        };

        assert_eq!(canvas.painters.is_empty(), true);

        canvas
            .add_painter("painter".to_string(), painter_1)
            .unwrap();

        assert_eq!(canvas.painters.len(), 1);

        let second_painter_add_result = canvas
            .add_painter("painter_two".to_string(), painter_2)
            .unwrap_err();

        assert_eq!(second_painter_add_result, AddPainterError::SpaceTaken);
        assert_eq!(canvas.painters.len(), 1);
    }
}
