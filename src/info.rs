pub fn sockets() -> usize {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoSockets) };
    ret as usize
}

pub fn cores() -> usize {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoCores) };
    ret as usize
}

pub fn threads() -> usize {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoThreads) };
    ret as usize
}

pub fn cache_l1() -> usize {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoCacheL1) };
    ret as usize
}

pub fn cache_l2() -> usize {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoCacheL2) };
    ret as usize
}

pub fn cache_l3() -> usize {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoCacheL3) };
    ret as usize
}

pub fn sse2() -> bool {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoSse2) };
    ret != 0
}

pub fn sse41() -> bool {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoSse41) };
    ret != 0
}

pub fn avx() -> bool {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoAvx) };
    ret != 0
}

pub fn avx2() -> bool {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoAvx2) };
    ret != 0
}

pub fn avx512f() -> bool {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoAvx512f) };
    ret != 0
}

pub fn avx512bw() -> bool {
    let ret =
        unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoAvx512bw) };
    ret != 0
}

pub fn avx512vnni() -> bool {
    let ret =
        unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoAvx512vnni) };
    ret != 0
}

pub fn vmx() -> bool {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoVmx) };
    ret != 0
}

pub fn vsx() -> bool {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoVsx) };
    ret != 0
}
pub fn neon() -> bool {
    let ret = unsafe { simdlib_sys::SimdCpuInfo(simdlib_sys::SimdCpuInfoType::SimdCpuInfoNeon) };
    ret != 0
}
