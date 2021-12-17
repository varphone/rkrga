//! RKRGA 应用接口 FFI 绑定。
//!
//! 当前基于 Rockchip RGA v1.2.x 应用接口接口实现。
//!
#![allow(deref_nullptr)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unaligned_references)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_deinit() {
        unsafe {
            assert_eq!(c_RkRgaInit(), 0);
            c_RkRgaDeInit();
        }
    }

    #[test]
    fn test_blit() {
        use std::ptr::{null, null_mut};

        unsafe {
            assert_ne!(0, c_RkRgaBlit(null_mut(), null_mut(), null_mut()));
        }
    }
}
