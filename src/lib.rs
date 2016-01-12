extern crate image;
extern crate num;

use image::{
    Rgb,
    Primitive,
};
use num::FromPrimitive;

fn scale<T: Primitive + FromPrimitive>(start: T, end: T, f: f64) -> T {
    let start : f64 = start.to_f64().unwrap();
    let end   : f64 =   end.to_f64().unwrap();
    let delta : f64 = end - start;
    let res   : f64 = start + delta * f;
    FromPrimitive::from_f64(res).unwrap()
}

pub fn scale_rgb<T: Primitive + FromPrimitive>(start: &Rgb<T>, end: &Rgb<T>, f: f64) -> Result<Rgb<T>, &'static str> {
    if f.is_nan() || f < 0.0_f64 || f > 1.0_f64 {
        if start == end {
            Ok(Rgb{
                data: [
                    start.data[0],
                    start.data[1],
                    start.data[2],
                ]
            })
        } else {
            Err("f is outside [0; 1]")
        }
    } else {
        Ok(Rgb{
            data: [
                scale(start.data[0], end.data[0], f),
                scale(start.data[1], end.data[1], f),
                scale(start.data[2], end.data[2], f),
            ]
        })
    }
}

#[test]
fn test_scaling_rgb_u8() {
    let white     : Rgb<u8> = Rgb{ data: [255, 255, 255] };
    let black     : Rgb<u8> = Rgb{ data: [  0,   0,   0] };
    let orange    : Rgb<u8> = Rgb{ data: [255, 127,   0] };
    let turquoise : Rgb<u8> = Rgb{ data: [  0, 127, 255] };

    /* invalid values for f */
    let r = scale_rgb(&white, &black, -0.5);
    assert!(r.is_err());
    let r = scale_rgb(&white, &black, 1.5);
    assert!(r.is_err());
    let r = scale_rgb(&white, &black, std::f64::INFINITY);
    assert!(r.is_err());
    let r = scale_rgb(&white, &black, std::f64::NAN);
    assert!(r.is_err());

    /* Accept if it's the same color */
    let r = scale_rgb(&white, &white, -0.5);
    assert!(r.is_ok());
    assert!(r.unwrap() == white);
    let r = scale_rgb(&white, &white, 1.5);
    assert!(r.is_ok());
    assert!(r.unwrap() == white);
    let r = scale_rgb(&white, &white, std::f64::INFINITY);
    assert!(r.is_ok());
    assert!(r.unwrap() == white);
    let r = scale_rgb(&white, &white, std::f64::NAN);
    assert!(r.is_ok());
    assert!(r.unwrap() == white);

    let r = scale_rgb(&white, &black, 0.5);
    assert!(r.is_ok());
    let e : Rgb<u8> = Rgb{ data: [127, 127, 127] };
    let r = r.unwrap();
    assert!(r == e);

    let r = scale_rgb(&black, &white, 0.5);
    assert!(r.is_ok());
    let e : Rgb<u8> = Rgb{ data: [127, 127, 127] };
    let r = r.unwrap();
    assert!(r == e);

    let r = scale_rgb(&orange, &orange, 0.5);
    assert!(r.is_ok());
    let r = r.unwrap();
    assert!(r == orange);

    let r = scale_rgb(&orange, &orange, 0.0);
    assert!(r.is_ok());
    let r = r.unwrap();
    assert!(r == orange);

    let r = scale_rgb(&orange, &orange, 1.0);
    assert!(r.is_ok());
    let r = r.unwrap();
    assert!(r == orange);

    let r = scale_rgb(&orange, &turquoise, 0.3);
    assert!(r.is_ok());
    let e : Rgb<u8> = Rgb{ data: [178, 127,  76] };
    let r = r.unwrap();
    assert!(r == e);

    let r = scale_rgb(&orange, &turquoise, 0.8);
    assert!(r.is_ok());
    let e : Rgb<u8> = Rgb{ data: [ 51, 127, 204] };
    let r = r.unwrap();
    assert!(r == e);

    let r = scale_rgb(&turquoise, &orange, 0.3);
    assert!(r.is_ok());
    let e : Rgb<u8> = Rgb{ data: [ 76, 127, 178] };
    let r = r.unwrap();
    assert!(r == e);

    let r = scale_rgb(&turquoise, &orange, 0.8);
    assert!(r.is_ok());
    let e : Rgb<u8> = Rgb{ data: [204, 127, 51] };
    let r = r.unwrap();
    assert!(r == e);
}

