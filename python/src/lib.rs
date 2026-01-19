use common::shapes::{Circle, Rectangle, ShapeEnum};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::pymethods;
use pyo3::types::{PyList, PyListMethods, PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, Python};

mod quadtree;

use crate::quadtree::{PyConfig, QuadTreeWrapper};

#[pymodule]
fn bolt_quadtree(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<QuadTreeWrapper>()?;
    m.add_class::<PyConfig>()?;

    m.add_class::<PyCircle>()?;
    m.add_class::<PyRectangle>()?;

    Ok(())
}

#[derive(Clone, Debug)]
#[pyclass(name = "Circle")]
pub struct PyCircle {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
    #[pyo3(get, set)]
    pub radius: f32,
}

#[pymethods]
impl PyCircle {
    #[new]
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        PyCircle { x, y, radius }
    }
}

#[derive(Clone, Debug)]
#[pyclass(name = "Rectangle")]
pub struct PyRectangle {
    rectangle: Rectangle,
}

#[pymethods]
impl PyRectangle {
    #[new]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let center_x = x + width / 2.0;
        let center_y = y + height / 2.0;
        PyRectangle {
            rectangle: Rectangle::new(center_x, center_y, width, height),
        }
    }

    #[getter]
    pub fn x(&self) -> f32 {
        self.rectangle.left()
    }

    #[setter]
    pub fn set_x(&mut self, x: f32) {
        self.rectangle.x = x + self.rectangle.width / 2.0;
    }

    #[getter]
    pub fn y(&self) -> f32 {
        self.rectangle.top()
    }

    #[setter]
    pub fn set_y(&mut self, y: f32) {
        self.rectangle.y = y + self.rectangle.height / 2.0;
    }

    #[getter]
    pub fn width(&self) -> f32 {
        self.rectangle.width()
    }

    #[getter]
    pub fn height(&self) -> f32 {
        self.rectangle.height()
    }

    pub fn left(&self) -> f32 {
        self.rectangle.left()
    }

    pub fn right(&self) -> f32 {
        self.rectangle.right()
    }

    pub fn top(&self) -> f32 {
        self.rectangle.top()
    }

    pub fn bottom(&self) -> f32 {
        self.rectangle.bottom()
    }

    pub fn top_left(&self) -> (f32, f32) {
        self.rectangle.top_left()
    }

    pub fn top_right(&self) -> (f32, f32) {
        self.rectangle.top_right()
    }

    pub fn bottom_left(&self) -> (f32, f32) {
        self.rectangle.bottom_left()
    }

    pub fn bottom_right(&self) -> (f32, f32) {
        self.rectangle.bottom_right()
    }

    pub fn distance_to_point(&self, x: f32, y: f32) -> f32 {
        self.rectangle.distance_to_point(x, y)
    }

    pub fn contains_circle(&self, x: f32, y: f32, radius: f32) -> bool {
        self.rectangle.contains_circle(x, y, radius)
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        self.rectangle.contains_point(x, y)
    }

    pub fn expand_to_include(&mut self, other: &PyRectangle) {
        self.rectangle.expand_to_include(&other.rectangle)
    }

    pub fn copy(&self) -> PyRectangle {
        PyRectangle {
            rectangle: self.rectangle.clone(),
        }
    }
}

fn extract_shape(py: Python, shape: Py<PyAny>) -> PyResult<ShapeEnum> {
    if let Ok(py_rectangle) = shape.extract::<PyRectangle>(py) {
        Ok(ShapeEnum::Rectangle(py_rectangle.rectangle))
    } else if let Ok(py_circle) = shape.extract::<PyCircle>(py) {
        Ok(ShapeEnum::Circle(Circle::new(
            py_circle.x,
            py_circle.y,
            py_circle.radius,
        )))
    } else {
        Err(PyTypeError::new_err(
            "Expected a Rectangle or Circle object",
        ))
    }
}

fn extract_entity_types(entity_types: Option<&Bound<'_, PyList>>) -> PyResult<Option<Vec<u32>>> {
    match entity_types {
        Some(entity_types_list) => {
            let et: Result<Vec<u32>, _> = entity_types_list
                .iter()
                .map(|item| item.extract::<u32>())
                .collect();
            Ok(Some(et?))
        }
        None => Ok(None),
    }
}
