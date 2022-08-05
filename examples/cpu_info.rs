fn main() {
    println!("Sockets : {}", simdlib::info::sockets());
    println!("Cores : {}", simdlib::info::cores());
    println!("Threads : {}", simdlib::info::threads());
    println!("L1D Cache : {}", simdlib::info::cache_l1() / 1024);
    println!("L2 Cache : {}", simdlib::info::cache_l2() / 1024);
    println!("L3 Cache : {}", simdlib::info::cache_l3() / 1024);
    println!("SSE2 : {}", simdlib::info::sse2());
    println!("SSE4.1 : {}", simdlib::info::sse41());
    println!("AVX : {}", simdlib::info::avx2());
    println!("AVX2 : {}", simdlib::info::avx2());
    println!("AVX-512F : {}", simdlib::info::avx512f());
    println!("AVX-512BW : {}", simdlib::info::avx512bw());
    println!("AVX-512VNNI : {}", simdlib::info::avx512vnni());
    println!("PowerPC-Altivec : {}", simdlib::info::vmx());
    println!("PowerPC-VSX : {}", simdlib::info::vsx());
    println!("ARM-NEON : {}", simdlib::info::neon());
}
