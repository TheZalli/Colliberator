use super::*;

#[test]
fn rgb_to_hsv() {
    let rgb = SRGB24Color::new(128, 255, 55);
    let hsv = rgb.into_float().hsv().normalize();
    let rgb2 = hsv.rgb().quantizate_u8();

    assert_eq!(rgb, rgb2);
}

#[test]
fn srgb_to_linear() {
    let srgb = SRGB24Color::new(128, 255, 55);
    let lin_rgb = srgb.into_float().decode();
    let srgb2 = lin_rgb.encode().quantizate_u8();

    assert_eq!(srgb, srgb2)
}

#[test]
fn srgb_to_linear_to_hsv() {
    let srgb = SRGB24Color::new(128, 255, 55);
    let lin_hsv = srgb.into_float().decode().hsv().normalize();
    let srgb2 = lin_hsv.rgb().encode().quantizate_u8();

    assert_eq!(srgb, srgb2)
}
