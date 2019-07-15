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