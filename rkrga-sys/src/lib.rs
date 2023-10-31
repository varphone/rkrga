//! RKRGA 应用接口 FFI 绑定。
//!
//! 当前基于 Rockchip RGA v1.2.x 应用接口接口实现。
//!
#![allow(deref_nullptr)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "use-bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[cfg(all(
    feature = "v1_2_0",
    not(any(feature = "use-bindgen", feature = "v1_2_2"))
))]
include!("bindings_v1_2_0.rs");
#[cfg(all(
    feature = "v1_2_2",
    not(any(feature = "use-bindgen", feature = "v1_2_3"))
))]
include!("bindings_v1_2_2.rs");
#[cfg(all(
    feature = "v1_2_3",
    not(any(feature = "use-bindgen", feature = "v1_2_4"))
))]
include!("bindings_v1_2_3.rs");
#[cfg(all(
    feature = "v1_2_4",
    not(any(feature = "use-bindgen", feature = "v1_2_5"))
))]
include!("bindings_v1_2_4.rs");
#[cfg(all(
    feature = "v1_2_5",
    not(any(feature = "use-bindgen", feature = "v1_2_6"))
))]
include!("bindings_v1_2_5.rs");
#[cfg(all(
    feature = "v1_2_6",
    not(any(feature = "use-bindgen", feature = "v1_3_0"))
))]
include!("bindings_v1_2_6.rs");
#[cfg(all(
    feature = "v1_3_0",
    not(any(feature = "use-bindgen", feature = "v1_3_1"))
))]
include!("bindings_v1_3_0.rs");
#[cfg(all(
    feature = "v1_3_1",
    not(any(feature = "use-bindgen", feature = "v1_4_0"))
))]
include!("bindings_v1_3_1.rs");
#[cfg(all(
    feature = "v1_4_0",
    not(any(feature = "use-bindgen", feature = "v1_6_0"))
))]
include!("bindings_v1_4_0.rs");
#[cfg(all(
    feature = "v1_6_0",
    not(any(feature = "use-bindgen", feature = "v1_7_2"))
))]
include!("bindings_v1_6_0.rs");
#[cfg(all(
    feature = "v1_7_2",
    not(any(feature = "use-bindgen", feature = "v1_8_0"))
))]
include!("bindings_v1_7_2.rs");
#[cfg(all(
    feature = "v1_8_0",
    not(any(feature = "use-bindgen", feature = "v1_9_0"))
))]
include!("bindings_v1_8_0.rs");
#[cfg(all(
    feature = "v1_9_0",
    not(any(feature = "use-bindgen", feature = "v1_9_2"))
))]
include!("bindings_v1_9_0.rs");
#[cfg(all(
    feature = "v1_9_2",
    not(any(feature = "use-bindgen", feature = "v1_9_3"))
))]
include!("bindings_v1_9_2.rs");
#[cfg(all(
    feature = "v1_9_3",
    not(any(feature = "use-bindgen", feature = "v1_10_0"))
))]
include!("bindings_v1_9_3.rs");

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
