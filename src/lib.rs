//! Raster Graphic Acceleration
//!
//! Rockchip RGA 是一个独立的二维光栅图形加速单元。
//! 它加速了二维图形操作，例如点/线绘制、图像缩放、旋转、位图、图像合成等。
//!
//! 当前基于 Rockchip RGA v1.2+ 应用接口接口实现。
//!
//! # Examples
//! ```
//! use rkrga::{Rga, RgaBuffer, RgaInfoBuilder, RgaRectBuilder, RgaTransform};
//! use std::sync::Arc;
//!
//! // 初始化 RGA 模块
//! let rga = Arc::new(Rga::new().unwrap());
//!
//! // 设定输入、输出图像信息
//! let src_rect = RgaRectBuilder::new().size(1280, 720).format(RgaPixelFormat::Rgba8888).build();
//! let dst_rect = RgaRectBuilder::new().size(720, 1280).format(RgaPixelFormat::Rgba8888).build();
//!
//! // 分配输入、输出内存缓冲区
//! let src_bo = RgaBuffer::with_rect_mapped(Arc::clone(&rga), &src_rect).unwrap();
//! let dst_bo = RgaBuffer::with_rect_mapped(Arc::clone(&rga), &dst_rect).unwrap();
//!
//! // 生成 RGA 操作信息
//! let src_info = RgaInfoBuilder::new().bo(&src_bo).rect(&src_rect).build();
//! let mut dst_info = RgaInfoBuilder::new().bo(&dst_bo).rect(&dst_rect).build();
//!
//! // 执行旋转操作
//! rga.rotate(&src_info, &mut dst_info, RgaTransform::Rot90).unwrap();
//! ```
use std::io;
use std::sync::Arc;

pub use rkrga_sys as ffi;

pub type RgaInfo = ffi::rga_info_t;
pub type RgaRect = ffi::rga_rect_t;

/// 一个描述 RGA 模块的类型。
#[derive(Debug)]
pub struct Rga;

impl Rga {
    /// 创建一个 RGA 对象实例。
    pub fn new() -> Result<Self, io::Error> {
        unsafe {
            match ffi::c_RkRgaInit() {
                0 => Ok(Self),
                err => Err(io::Error::from_raw_os_error(err)),
            }
        }
    }

    /// 分配一个内存缓冲对象。
    pub fn alloc_buffer(
        self: &Arc<Self>,
        width: i32,
        height: i32,
        bpp: i32,
    ) -> Result<RgaBuffer, io::Error> {
        RgaBuffer::new(Arc::clone(self), width, height, bpp)
    }

    /// 对象图像进行比特操作。
    pub fn blit(
        &self,
        src: &RgaInfo,
        dst: &mut RgaInfo,
        extra_src: Option<&mut RgaInfo>,
    ) -> Result<(), io::Error> {
        unsafe {
            let mut src = *src;
            let extra_src = extra_src
                .map(|x| x as *mut RgaInfo)
                .unwrap_or(std::ptr::null_mut());
            match ffi::c_RkRgaBlit(&mut src, dst, extra_src) {
                0 => Ok(()),
                err => Err(io::Error::from_raw_os_error(err)),
            }
        }
    }

    /// 对象图像进行色彩空间转换操作。
    pub fn csc(&self, src: &RgaInfo, dst: &mut RgaInfo) -> Result<(), io::Error> {
        self.blit(src, dst, None)
    }

    /// 使用指定颜色填充图像。
    pub fn fill(&self, dst: &RgaInfo, color: u32) -> Result<(), io::Error> {
        unsafe {
            let mut dst = RgaInfo {
                color: color as i32,
                ..*dst
            };
            match ffi::c_RkRgaColorFill(&mut dst) {
                0 => Ok(()),
                err => Err(io::Error::from_raw_os_error(err)),
            }
        }
    }

    /// 对象图像进行旋转操作。
    pub fn rotate(
        &self,
        src: &RgaInfo,
        dst: &mut RgaInfo,
        trans: RgaTransform,
    ) -> Result<(), io::Error> {
        let src = RgaInfo {
            rotation: trans as i32,
            ..*src
        };
        self.blit(&src, dst, None)
    }

    /// 对象图像进行缩放操作。
    pub fn scale(&self, src: &RgaInfo, dst: &mut RgaInfo) -> Result<(), io::Error> {
        self.blit(src, dst, None)
    }
}

impl Default for Rga {
    fn default() -> Self {
        Self::new().expect("Rga::new() failed")
    }
}

impl Drop for Rga {
    fn drop(&mut self) {
        unsafe {
            ffi::c_RkRgaDeInit();
        }
    }
}

/// 一个描述 RGA 颜色空间模式的枚举。
#[repr(i32)]
pub enum RgaColorSpaceMode {
    /// YUV -> RGB BT.601 MPEG
    Yuv2RgbMode0 = 0x0000,
    /// YUV -> RGB BT.601 JPEG
    Yuv2RgbMode1 = 0x0001,
    /// YUV -> RGB BT.709
    Yuv2RgbMode2 = 0x0002,
    /// RGB -> YUV BT.601
    RgbToYuv601Full = 0x0100,
    /// RGB -> YUV BT.709
    RgbToYuv709Full = 0x0200,
    ///
    Yuv2Yuv601Limit_709Limit = 0x0300,
    Yuv2Yuv601Limit_709Full = 0x0400,
    Yuv2Yuv709Limit_601Limit = 0x0500,
    Yuv2Yuv709Limit_601Full = 0x0600,
    Yuv2Yuv601Full_709Limit = 0x0700,
    Yuv2Yuv601Full_709Full = 0x0800,
    Yuv2Yuv709Full_601Limit = 0x0900,
    Yuv2Yuv709Full_601Full = 0x0a00,
}

/// 一个描述 RGA 位操作的枚举。
#[repr(i32)]
pub enum RgaRop {
    /// DEST=(SRC AND DEST).
    SrcAndDest = 0x88,
    /// DEST=(SRC OR DEST).
    SrcOrDest = 0xee,
    /// DEST=NOT(DEST).
    NotDest = 0x55,
    /// DEST=NOT(SRC).
    NotSrc = 0x33,
    /// DEST=(SRC XOR DEST).
    SrcXorDest = 0xf6,
    /// DEST=NOT(SRC XOR DEST).
    NotSrcXorDest = 0xf9,
}

/// 一个描述 RGA 旋转或翻转操作的枚举。
#[repr(i32)]
pub enum RgaTransform {
    /// 水平翻转。
    FlipH = ffi::HAL_TRANSFORM_FLIP_H as i32,
    /// 水平及垂直翻转。
    FlipHV = ffi::HAL_TRANSFORM_FLIP_H_V as i32,
    /// 垂直翻转。
    FlipV = ffi::HAL_TRANSFORM_FLIP_V as i32,
    /// 旋转 90 度。
    Rot90 = ffi::HAL_TRANSFORM_ROT_90 as i32,
    /// 旋转 180 度。
    Rot180 = ffi::HAL_TRANSFORM_ROT_180 as i32,
    /// 旋转 270 度。
    Rot270 = ffi::HAL_TRANSFORM_ROT_270 as i32,
}

mod bo;
mod builder;
mod pixfmt;

pub use bo::*;
pub use builder::*;
pub use pixfmt::*;
