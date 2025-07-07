#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod ffx_fsr2;

#[test]
fn test_ffx_fsr2() {
    use crate::ffx_fsr2::ffxFsr2GetJitterPhaseCount;
    unsafe {
        let jitter = ffxFsr2GetJitterPhaseCount(1920, 1920);
        println!("Jitter Phase Count: {}", jitter);
    }
}
