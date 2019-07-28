use super::*;

#[test]
fn rgb_to_hsv() {
    let rgb = SRGB24Color::new(128, 255, 55);
    let hsv = rgb.float().hsv().normalize();
    let rgb2 = hsv.rgb().conv();

    assert_eq!(rgb, rgb2);
}

#[test]
fn srgb_to_linear() {
    let srgb = SRGB24Color::new(128, 255, 55);
    let lin_rgb = srgb.float().std_decode();
    let srgb2 = lin_rgb.std_encode().conv();

    assert_eq!(srgb, srgb2)
}

#[test]
fn srgb_to_linear_to_hsv() {
    let srgb = SRGB24Color::new(128, 255, 55);
    let lin_hsv = srgb.float().std_decode().hsv().normalize();
    let srgb2 = lin_hsv.rgb().std_encode().conv();

    assert_eq!(srgb, srgb2)
}

#[test]
fn hex_conversion() {
    for hex in (0..=0xFFFFFF).step_by(30_000) {
        let hex_str: String = format!("{:06X}", hex);
        let color = unsafe {
            SRGB24Color::from_hex_unchecked(hex_str.clone().into_boxed_str())
        };
        let hex_str2 = format!("{:X}", color);

        assert_eq!(hex_str, hex_str2);
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
