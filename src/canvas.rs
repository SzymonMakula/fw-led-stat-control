use std::collections::HashMap;
use std::fs;

use crate::config::Config;
use crate::matrix::{EMPTY_MATRIX, Matrix, MATRIX_WIDTH};
use crate::picture::Picture;
use crate::plugin::Plugin;
use crate::wasm_module::WasmModule;

pub struct Canvas {
    pub(crate) painters: HashMap<String, Painter>,
}
impl From<Config> for Canvas {
    fn from(value: Config) -> Self {
        let painters = value
            .plugins
            .into_iter()
            .filter_map(|record| {
                if record.pos_x.is_some() && record.pos_y.is_some() {
                    fs::read(record.path)
                        .ok()
                        .map(WasmModule::new)
                        .map(Plugin::from)
                        .map(|plugin: Plugin| Painter {
                            plugin,
                            offset_x: record.pos_x.unwrap(),
                            offset_y: record.pos_y.unwrap(),
                        })
                        .map(|painter| (record.name, painter))
                } else {
                    None
                }
            })
            .collect::<HashMap<String, Painter>>();

        Self { painters }
    }
}

pub(crate) struct Painter {
    offset_x: usize,
    offset_y: usize,
    plugin: Plugin,
}

impl Painter {
    // Returns space taken by Picture as Matrix. non 0 values indicate space taken.
    fn get_space_as_matrix(&self) -> Matrix {
        let mut output = Vec::from(EMPTY_MATRIX);

        for row in 0..self.plugin.image_height {
            for col in 0..self.plugin.image_width {
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
    DuplicateIdentifier,
}
impl Canvas {
    pub(crate) fn new() -> Self {
        Self {
            painters: HashMap::new(),
        }
    }

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
    pub fn add_painter(&mut self, painter: Painter) -> Result<(), AddPainterError> {
        if self.painters.get(&painter.plugin.name).is_some() {
            return Err(AddPainterError::DuplicateIdentifier);
        }

        let is_vacant = self.is_space_vacant(&painter);

        if !is_vacant {
            return Err(AddPainterError::SpaceTaken);
        }

        self.painters.insert(painter.plugin.name.clone(), painter);
        Ok(())
    }

    // Call .draw() for all Painters and return the resulting Matrix
    pub fn paint_matrix(&mut self) -> Matrix {
        self.painters
            .iter_mut()
            .map(|(_, painter)| {
                painter
                    .plugin
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
        for row in 0..painter.plugin.image_height {
            for col in 0..painter.plugin.image_width {
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
    use crate::plugin::Plugin;

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
            plugin: Plugin {
                image_width: 5,
                image_height: 3,
                drawer: Box::new(PainterMock {}),
                description: "test".to_string(),
                name: "test".to_string(),
            },
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
    use crate::plugin::Plugin;

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
            offset_x: 0,
            offset_y: 0,
            plugin: Plugin {
                image_width: 2,
                image_height: 2,
                drawer: Box::new(PainterMock {}),
                description: "test".to_string(),
                name: "test".to_string(),
            },
        };
        let painter_2 = Painter {
            offset_x: 2,
            offset_y: 2,
            plugin: Plugin {
                image_width: 2,
                image_height: 2,
                drawer: Box::new(PainterMock {}),
                description: "test".to_string(),
                name: "test2".to_string(),
            },
        };
        let painter_3 = Painter {
            offset_x: 4,
            offset_y: 4,
            plugin: Plugin {
                image_width: 2,
                image_height: 2,
                drawer: Box::new(PainterMock {}),
                description: "test".to_string(),
                name: "test3".to_string(),
            },
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
            offset_x: 0,
            offset_y: 0,
            plugin: Plugin {
                image_width: 2,
                image_height: 2,
                drawer: Box::new(PainterMock {}),
                description: "test".to_string(),
                name: "test".to_string(),
            },
        };
        let painter_2 = Painter {
            offset_x: 2,
            offset_y: 0,
            plugin: Plugin {
                image_width: 2,
                image_height: 2,
                drawer: Box::new(PainterMock {}),
                description: "test".to_string(),
                name: "test".to_string(),
            },
        };
        let canvas = Canvas {
            painters: HashMap::from([("painter_one".to_string(), painter_1)]),
        };
        assert_eq!(canvas.is_space_vacant(&painter_2), true)
    }

    #[test]
    fn is_space_vacant_when_busy() {
        let painter_1 = Painter {
            offset_x: 0,
            offset_y: 0,
            plugin: Plugin {
                image_width: 2,
                image_height: 2,
                drawer: Box::new(PainterMock {}),
                description: "test".to_string(),
                name: "test".to_string(),
            },
        };
        let painter_2 = Painter {
            offset_x: 1, // Intersects by two points
            offset_y: 0,
            plugin: Plugin {
                image_width: 2,
                image_height: 2,
                drawer: Box::new(PainterMock {}),
                description: "test".to_string(),
                name: "test".to_string(),
            },
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
            offset_x: 0,
            offset_y: 0,
            plugin: Plugin {
                image_width: 2,
                image_height: 2,
                drawer: Box::new(Painter1 {}),
                description: "test".to_string(),
                name: "test".to_string(),
            },
        };
        let painter_2 = Painter {
            offset_x: 2,
            offset_y: 0,
            plugin: Plugin {
                image_width: 2,
                image_height: 2,
                drawer: Box::new(Painter2 {}),
                description: "test".to_string(),
                name: "test".to_string(),
            },
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
            offset_x: 0,
            offset_y: 0,
            plugin: Plugin {
                image_width: 2,
                image_height: 2,
                drawer: Box::new(PainterMock {}),
                description: "test".to_string(),
                name: "test".to_string(),
            },
        };

        let mut canvas = Canvas {
            painters: HashMap::from([]),
        };

        assert_eq!(canvas.painters.is_empty(), true);

        canvas.add_painter(painter).unwrap();

        assert_eq!(canvas.painters.len(), 1)
    }

    #[test]
    fn add_painter_rejects_when_space_taken() {
        let painter_1 = Painter {
            offset_x: 0,
            offset_y: 0,
            plugin: Plugin {
                image_width: 2,
                image_height: 2,
                drawer: Box::new(PainterMock {}),
                description: "test".to_string(),
                name: "test".to_string(),
            },
        };
        // Intersects with painter_1 boundaries
        let painter_2 = Painter {
            offset_x: 1,
            offset_y: 1,
            plugin: Plugin {
                image_width: 4,
                image_height: 4,
                drawer: Box::new(PainterMock {}),
                description: "test".to_string(),
                name: "test2".to_string(),
            },
        };

        let mut canvas = Canvas {
            painters: HashMap::from([]),
        };

        assert_eq!(canvas.painters.is_empty(), true);

        canvas.add_painter(painter_1).unwrap();

        assert_eq!(canvas.painters.len(), 1);

        let second_painter_add_result = canvas.add_painter(painter_2).unwrap_err();

        assert_eq!(second_painter_add_result, AddPainterError::SpaceTaken);
        assert_eq!(canvas.painters.len(), 1);
    }
}
