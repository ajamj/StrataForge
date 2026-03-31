use pyo3::prelude::*;
use numpy::{PyArrayDyn, PyArrayMethods};
use ndarray::ArrayViewD;

/// High-performance zero-copy bridge to Python.
#[cfg(feature = "python")]
pub fn share_with_python<'py>(
    py: Python<'py>,
    data: &[f32],
    shape: Vec<usize>,
) -> PyResult<Bound<'py, PyArrayDyn<f32>>> {
    let view = ArrayViewD::from_shape(shape, data)
        .map_err(|e: ndarray::ShapeError| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    
    // Safety: The caller must ensure the data slice outlives the NumPy array
    let array = unsafe {
        PyArrayDyn::borrow_from_array_bound(&view, py.None().into_bound(py))
    };
    Ok(array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "python")]
    fn test_zero_copy_bridge() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let data = vec![1.0, 2.0, 3.0, 4.0];
            let shape = vec![4];
            let array = share_with_python(py, &data, shape).unwrap();
            
            // Check that the array has the correct values
            let readonly = array.readonly();
            let view = readonly.as_array();
            assert_eq!(view[0], 1.0);
            assert_eq!(view[3], 4.0);
            
            // Verify that the memory address is the same (zero-copy)
            let ptr = data.as_ptr() as usize;
            let data_ptr = array.data() as usize;
            assert_eq!(ptr, data_ptr);
        });
    }
}
