#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::env;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::*;
    use std::ptr;
    use std::slice;
    const EPSILON: tsReal = 0.0001;
    #[test]
    fn interpolation() {
        // adapted from https://github.com/msteinbeck/tinyspline/blob/master/test/c/interpolation.c
        unsafe {
            let points: [tsReal; 10] = [
                 1.0,
                -1.0,
                -1.0,
                 2.0,
                 1.0,
                 4.0,
                 4.0,
                 3.0,
                 7.0,
                 5.0,
            ];
            let mut ctrlp: *mut tsReal = ptr::null_mut();
            let mut knots: *mut tsReal = ptr::null_mut();
            let mut spline = ts_bspline_init();
            let mut status = tsStatus {
                code: tsError_TS_SUCCESS,
                message: [0; 100]
            };
            ts_bspline_interpolate_cubic_natural(
                &points as *const tsReal, 5, 2,
                &mut spline as *mut tsBSpline,
                &mut status as *mut tsStatus
            );
            if status.code != tsError_TS_SUCCESS {
                panic!("Error on ts_bspline_interpolate_cubic_natural");
            }
            ts_bspline_control_points(
                &mut spline as *mut tsBSpline,
                &mut ctrlp as *mut *mut tsReal,
                &mut status as *mut tsStatus
            );
            if status.code != tsError_TS_SUCCESS {
                panic!("Error on ts_bspline_control_points");
            }
            let numpts = ts_bspline_num_control_points(&mut spline as *mut tsBSpline);
            if numpts != 16 {
                panic!("Number of control points does not equal 16 (got {})", numpts);
            }
            let target_points = [
                 1.0,
                -1.0,
                 0.0,
                 0.0,
                -1.0,
                 1.0,
                -1.0,
                 2.0,
                -1.0,
                 2.0,
                -1.0,
                 3.0,
                 0.0,
                 4.0,
                 1.0,
                 4.0,
                 1.0,
                 4.0,
                 2.0,
                 4.0,
                 3.0,
                 3.0,
                 4.0,
                 3.0,
                 4.0,
                 3.0,
                 5.0,
                 3.0,
                 6.0,
                 4.0,
                 7.0,
                 5.0
            ];
            let ctrlp = std::slice::from_raw_parts(ctrlp, 32);
            for (i, value) in target_points.iter().enumerate() {
                if (value - ctrlp[i]).abs() > EPSILON {
                    panic!("Mismatch control point at index {}: {}, {}", i, value, ctrlp[i]);
                }
            }
            ts_bspline_knots(
                &mut spline as *mut tsBSpline,
                &mut knots as *mut *mut tsReal,
                &mut status as *mut tsStatus
            );
            if status.code != tsError_TS_SUCCESS {
                panic!("Error on ts_bspline_knots");
            }
            let target_knots = [
                0.0, 
                0.0, 
                0.0, 
                0.0, 
                0.25,
                0.25,
                0.25,
                0.25,
                0.5, 
                0.5, 
                0.5, 
                0.5, 
                0.75,
                0.75,
                0.75,
                0.75,
                1.0, 
                1.0, 
                1.0, 
                1.0, 
            ];
            let knots = slice::from_raw_parts(knots, 20);
            for (i, value) in target_knots.iter().enumerate() {
                if (value - knots[i]).abs() > EPSILON {
                    panic!("Mismatch knot at index {}: {}, {}", i, value, ctrlp[i]);
                }
            }
            ts_bspline_free(&mut spline as *mut tsBSpline);
        }
    }
}