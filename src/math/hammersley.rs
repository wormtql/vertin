use cgmath::Vector2;

pub fn radical_inverse_vdc(bits: usize) -> f64 {
    let mut bits = bits;

    bits = (bits << 16) | (bits >> 16);
    bits = ((bits & 0x55555555) << 1) | ((bits & 0xAAAAAAAA) >> 1);
    bits = ((bits & 0x33333333) << 2) | ((bits & 0xCCCCCCCC) >> 2);
    bits = ((bits & 0x0F0F0F0F) << 4) | ((bits & 0xF0F0F0F0) >> 4);
    bits = ((bits & 0x00FF00FF) << 8) | ((bits & 0xFF00FF00) >> 8);

    bits as f64 * 2.3283064365386963e-10
}

pub fn hammersley(i: usize, n: usize) -> Vector2<f64> {
    Vector2::new(
        i as f64 / n as f64,
        radical_inverse_vdc(i)
    )
}
