#[inline(always)]
pub fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
    assert!(a.len() == b.len(), "All arrays must have the same size.");
    let mut distance = 0.0f32;
    unsafe {
        simdlib_sys::SimdCosineDistance32f(a.as_ptr(), b.as_ptr(), a.len() as _, &mut distance);
    }
    distance
}
