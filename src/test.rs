use crate::*;

#[test]
fn rgb_to_hsv() {
    let rgb = SRGB24Color::new(128, 255, 55);
    let hsv = rgb.conv::<f32>().hsv::<Deg<f32>>().normalize();
    let rgb2 = hsv.rgb().conv();

    assert_eq!(rgb, rgb2);
}

#[test]
fn srgb_to_linear() {
    let srgb = SRGB24Color::new(128, 255, 55);
    let lin_rgb = srgb.conv::<f32>().std_decode();
    let srgb2 = lin_rgb.std_encode().conv();

    assert_eq!(srgb, srgb2)
}

#[test]
fn srgb_to_linear_to_hsv() {
    let srgb = SRGB24Color::new(128, 255, 55);
    let lin_hsv = srgb
        .conv::<f32>()
        .std_decode()
        .hsv::<Deg<f32>>()
        .normalize();
    let srgb2 = lin_hsv.rgb().std_encode().conv();

    assert_eq!(srgb, srgb2)
}

#[test]
fn hex_conversion_6char() {
    for hex in (0..=0xFF_FF_FF).step_by(30_000) {
        let hex_str = format!("{:06X}", hex);

        let color_safe = SRGB24Color::from_hex(&hex_str).unwrap();
        let color = unsafe { SRGB24Color::from_hex_unchecked(hex_str.clone()) };

        let hex_str_safe = format!("{:X}", color_safe);
        let hex_str_unsafe = format!("{:X}", color);

        assert_eq!(hex_str, hex_str_safe);
        assert_eq!(hex_str, hex_str_unsafe);
    }
}
#[test]
fn hex_conversion_3char() {
    for hex in 0u16..=0xFFF {
        let hex_str3 = format!("{:03X}", hex);

        let mut i = hex_str3.chars();
        let hex_str6 = format!(
            "{0}{0}{1}{1}{2}{2}",
            i.next().unwrap(),
            i.next().unwrap(),
            i.next().unwrap()
        );

        let color_safe = SRGB24Color::from_hex(&hex_str3).unwrap();
        let color = unsafe { SRGB24Color::from_hex_unchecked(hex_str3) };

        let hex_str_safe = format!("{:X}", color_safe);
        let hex_str_unsafe = format!("{:X}", color);

        assert_eq!(hex_str6, hex_str_safe);
        assert_eq!(hex_str6, hex_str_unsafe);
    }
}

#[test]
fn into_iterator() {
    let c1 = SRGBAColor::new((0.25, 0.5, 1.0), 0.9);
    let c2 = LinRGB48Color::new(255, 8, 240);
    let mut i1 = c1.into_iter();
    let mut i2 = c2.into_iter();

    assert_eq!(i1.next(), Some(0.25));
    assert_eq!(i1.next(), Some(0.5));
    assert_eq!(i1.next(), Some(1.0));
    assert_eq!(i1.next(), Some(0.9));
    assert_eq!(i1.next(), None);

    assert_eq!(i2.next(), Some(255));
    assert_eq!(i2.next(), Some(8));
    assert_eq!(i2.next(), Some(240));
    assert_eq!(i2.next(), None);
}

#[test]
fn angle_conversion() {
    use std::f32::consts::PI;
    for i in 0..=999 {
        let f = (i as f32) / 1000.0;
        let deg: Deg<f32> = f.conv();
        let rad: Rad = f.conv();

        assert_eq!(deg.0, f * 360.0);
        assert_eq!(rad.0, f * 2.0 * PI);
        assert_eq!(deg.0.round(), rad.conv::<Deg<f32>>().0.round());
    }
}

#[test]
fn normalization() {
    use std::f32::{INFINITY, NEG_INFINITY};
    let rgba = SRGBAColor::new((2.0, -10.0, NEG_INFINITY), INFINITY);
    let hsv1 = StdHSVColor::new(-90.0, 2.0, -5.0);
    let hsv2 = StdHSVColor::new(400.0, 2.0, -5.0);
    let hsv3 = StdHSVColor::new(60.0, 0.0, 0.5);
    let hsv4 = StdHSVColor::new(0.0, 0.9, 0.2);
    let hsv5 = StdHSVColor::new(50.0, 0.25, 0.0);
    let hsv6 = StdHSVColor::new(0.0, 0.0, 0.8);
    let hsv7 = StdHSVColor::from([45.0, 0.0, 0.0]);

    assert_eq!(rgba.tuple(), (1.0, 0.0, 0.0, 1.0));
    assert_eq!(hsv1.tuple(), (Deg(270.0), 1.0, 0.0));
    assert_eq!(hsv2.tuple(), (Deg(40.0), 1.0, 0.0));
    assert_eq!(hsv3.tuple(), (Deg(0.0), 0.0, 0.5));
    assert_eq!(hsv4.tuple(), (Deg(0.0), 0.9, 0.2));
    assert_eq!(hsv5.tuple(), (Deg(0.0), 0.0, 0.0));
    assert_eq!(hsv6.array::<f32>(), [0.0, 0.0, 0.8]);
    assert_eq!(hsv7.tuple(), (0.0.into(), 0.0, 0.0));
}
