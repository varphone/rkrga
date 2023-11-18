use super::{
    ffi, RgaBuffer, RgaColorSpaceMode, RgaInfo, RgaPixelFormat, RgaRect, RgaRop, RgaTransform,
};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;

/// 一个描述 RGA 图像区域构建器的类型。
pub struct RgaRectBuilder {
    offset: Option<(i32, i32)>,
    size: Option<(i32, i32)>,
    stride: Option<(i32, i32)>,
    format: Option<RgaPixelFormat>,
}

impl RgaRectBuilder {
    /// 创建一个新的 RGA 图像区域构建器。
    pub fn new() -> Self {
        Self {
            offset: None,
            size: None,
            stride: None,
            format: None,
        }
    }

    /// 设置图像偏移。
    pub fn offset(mut self, x: i32, y: i32) -> Self {
        self.offset = Some((x, y));
        self
    }

    /// 设置图像大小。
    pub fn size(mut self, w: i32, h: i32) -> Self {
        self.size = Some((w, h));
        self
    }

    /// 设置图像内部大小。
    pub fn stride(mut self, w: i32, h: i32) -> Self {
        self.stride = Some((w, h));
        self
    }

    /// 设置图像像素格式。
    pub fn format(mut self, fmt: RgaPixelFormat) -> Self {
        self.format = Some(fmt);
        self
    }

    /// 生成适用于 RGA 的图像区域信息。
    pub fn build(self) -> RgaRect {
        let (w, h) = self
            .size
            .expect("RgaRectBuilder: `size` could not be empty!");
        let fmt = self
            .format
            .expect("RgaRectBuilder: `format` could not be empty!");
        let (x, y) = self.offset.unwrap_or_default();
        let (ws, hs) = self.stride.unwrap_or((w, h));

        RgaRect {
            xoffset: x,
            yoffset: y,
            width: w,
            height: h,
            wstride: ws,
            hstride: hs,
            format: ffi::RgaSURF_FORMAT::from(fmt) as i32,
            size: 0,
        }
    }
}

impl Default for RgaRectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 一个描述 RGA 操作信息构建器的类型。
pub struct RgaInfoBuilder<'a> {
    bo: Option<&'a RgaBuffer>,
    vir_addr: Option<*mut u8>,
    rect: Option<&'a RgaRect>,
    blend: Option<u32>,
    color: Option<i32>,
    format: Option<RgaPixelFormat>,
    color_space_mode: Option<RgaColorSpaceMode>,
    rop: Option<RgaRop>,
    rotation: Option<RgaTransform>,
}

impl<'a> RgaInfoBuilder<'a> {
    /// 创建一个新的 RGA 操作信息构建器。
    pub fn new() -> Self {
        Self {
            bo: None,
            vir_addr: None,
            rect: None,
            blend: None,
            color: None,
            format: None,
            color_space_mode: None,
            rop: None,
            rotation: None,
        }
    }

    /// 设置目标内存为 RGA 缓冲对象。
    pub fn bo(mut self, bo: &'a RgaBuffer) -> Self {
        self.bo = Some(bo);
        self
    }

    /// 设置目标内存为虚拟地址。
    pub fn vir_addr(mut self, vir_addr: *mut u8) -> Self {
        self.vir_addr = Some(vir_addr);
        self
    }

    /// 设置图像区域信息。
    pub fn rect(mut self, rect: &'a RgaRect) -> Self {
        self.rect = Some(rect);
        self
    }

    /// 设置混合参数。
    pub fn blend(mut self, blend: u32) -> Self {
        self.blend = Some(blend);
        self
    }

    /// 设置填充颜色。
    pub fn color(mut self, color: i32) -> Self {
        self.color = Some(color);
        self
    }

    /// 设置像素格式。
    pub fn format(mut self, format: RgaPixelFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// 设置颜色空间转换模式。
    pub fn color_space_mode(mut self, mode: RgaColorSpaceMode) -> Self {
        self.color_space_mode = Some(mode);
        self
    }

    /// 设置位操作模式。
    pub fn rop(mut self, rop: RgaRop) -> Self {
        self.rop = Some(rop);
        self
    }

    /// 设置旋转模式。
    pub fn rotation(mut self, rotation: RgaTransform) -> Self {
        self.rotation = Some(rotation);
        self
    }

    /// 构建 RGA 操作信息。
    pub fn build(self) -> RgaInfoRef<'a> {
        let mut info: RgaInfo = Default::default();

        if let Some(bo) = self.bo {
            info.fd = bo.dma_fd().unwrap_or(-1);
            info.virAddr = unsafe { bo.ptr() as *mut c_void };
            info.hnd = bo.handle();
            info.mmuFlag = 1;
        } else if let Some(vir_addr) = self.vir_addr {
            info.fd = -1;
            info.virAddr = vir_addr as *mut c_void;
            info.mmuFlag = 1;
        } else {
            panic!("RgaInfoBuilder: one of bo() orvir_addr() must be set!");
        }

        if let Some(rect) = self.rect {
            info.format = rect.format;
            info.rect = *rect;
        }

        if let Some(blend) = self.blend {
            info.blend = blend;
        }

        if let Some(color) = self.color {
            info.color = color;
        }

        if let Some(format) = self.format {
            info.format = ffi::RgaSURF_FORMAT::from(format) as i32;
        }

        if let Some(mode) = self.color_space_mode {
            info.color_space_mode = mode as i32;
        }

        if let Some(rop) = self.rop {
            info.rop_code = rop as i32;
        }

        if let Some(rotation) = self.rotation {
            info.rotation = rotation as i32;
        }

        #[cfg(feature = "v1_4_0")]
        {
            info.in_fence_fd = -1;
            info.out_fence_fd = -1;
        }

        info.sync_mode = ffi::RGA_BLIT_SYNC as i32;

        RgaInfoRef::new(info)
    }
}

impl<'a> Default for RgaInfoBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// 一个描述 RGA 操作信息引用的类型。
pub struct RgaInfoRef<'a> {
    info: RgaInfo,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> RgaInfoRef<'a> {
    /// 创建一个新的 RGA 操作信息引用。
    pub fn new(info: RgaInfo) -> Self {
        Self {
            info,
            _phantom: PhantomData,
        }
    }
}

impl<'a> Deref for RgaInfoRef<'a> {
    type Target = RgaInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl<'a> DerefMut for RgaInfoRef<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}
