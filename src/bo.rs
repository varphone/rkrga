//! 内存缓冲对象。
//!
use super::{ffi, Rga, RgaPixelFormat, RgaRect};
use std::io;
use std::sync::Arc;

/// 一个描述 RGA 内存缓冲对象的类型。
#[derive(Debug, Default)]
pub struct RgaBuffer {
    bo: ffi::bo_t,
    mapped: bool,
    _rga: Arc<Rga>,
}

impl RgaBuffer {
    /// 创建一个新的 RGA 内存缓冲对象。
    ///
    /// # Examples
    /// ```
    /// use rkrga::{Rga, RgaBuffer};
    /// use std::sync::Arc;
    ///
    /// let rga = Arc::new(Rga::new().unwrap());
    /// let mut bo = RgaBuffer::new(Arc::clone(&rga), 1920, 1080, 32); // 1920x1080 RGBA8888
    /// bo.map().unwrap();
    /// assert_eq!(bo.len(), 1920 * 1080 * 4);
    /// assert_eq!(bo.as_ptr(), std::ptr::null());
    /// bo.unmap();
    /// ```
    pub fn new(rga: Arc<Rga>, width: i32, height: i32, bpp: i32) -> Result<Self, io::Error> {
        unsafe {
            let mut bo: ffi::bo_t = Default::default();
            match ffi::c_RkRgaGetAllocBuffer(&mut bo, width, height, bpp) {
                0 => Ok(Self {
                    bo,
                    mapped: false,
                    _rga: rga,
                }),
                err => Err(io::Error::from_raw_os_error(err)),
            }
        }
    }

    /// 创建一个新的且已映射到用户空间的 RGA 内存缓冲对象。
    pub fn new_mapped(rga: Arc<Rga>, width: i32, height: i32, bpp: i32) -> Result<Self, io::Error> {
        let mut bo = Self::new(rga, width, height, bpp)?;
        bo.map()?;
        Ok(bo)
    }

    /// 为指定的 RGA 图像区域创建一个 RGA 内存缓冲对象。
    pub fn with_rect(rga: Arc<Rga>, rect: &RgaRect) -> Result<Self, io::Error> {
        let pixfmt = RgaPixelFormat::from(unsafe {
            std::mem::transmute::<i32, ffi::RgaSURF_FORMAT>(rect.format)
        });
        Self::new(rga, rect.width, rect.height, pixfmt.bits_per_pixel() as i32)
    }

    /// 为指定的 RGA 图像区域创建一个 RGA 内存缓冲对象。
    pub fn with_rect_mapped(rga: Arc<Rga>, rect: &RgaRect) -> Result<Self, io::Error> {
        let pixfmt = RgaPixelFormat::from(unsafe {
            std::mem::transmute::<i32, ffi::RgaSURF_FORMAT>(rect.format)
        });
        Self::new_mapped(rga, rect.width, rect.height, pixfmt.bits_per_pixel() as i32)
    }

    /// 返回文件描述符。
    ///
    /// # Note
    /// 此文件描述符并不能用于 DMA 操作，用于 DMA 操作的请使用 [`dma_fd`] 返回值。
    ///
    /// [`dma_fd`]: RgaBuffer::dma_fd()
    ///
    pub fn fd(&self) -> i32 {
        self.bo.fd
    }

    /// 返回资源句柄。
    pub fn handle(&self) -> u32 {
        self.bo.handle
    }

    /// 返回有效数据的偏移位置。
    pub fn offset(&self) -> usize {
        self.bo.offset as usize
    }

    /// 返回一行数据的字节跨度。
    pub fn pitch(&self) -> usize {
        self.bo.pitch as usize
    }

    /// 返回映射到用户空间的地址。
    ///
    /// # Note
    /// 当用户未进行映射操作时，会返回 NULL。
    ///
    /// # Safety
    /// 请小心使用此指针。
    pub unsafe fn ptr(&self) -> *mut u8 {
        self.bo.ptr as *mut u8
    }

    /// 返回已分配的内存块字节数。
    pub fn size(&self) -> usize {
        self.bo.size as usize
    }

    /// 返回字节数组。
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let ptr = self.ptr();
            let len = self.len();
            if !ptr.is_null() && len > 0 {
                std::slice::from_raw_parts(ptr, len)
            } else {
                &[]
            }
        }
    }

    /// 返回指定类型的映射到用户空间的只读地址。
    pub fn as_ptr<T>(&self) -> *const T {
        unsafe { self.ptr() as *const T }
    }

    /// 返回指定类型的映射到用户空间的可读写地址。
    pub fn as_mut_ptr<T>(&mut self) -> *mut T {
        unsafe { self.ptr() as *mut T }
    }

    /// 返回指定类型的映射到用户空间的只读数组切片。
    pub fn as_slice<T>(&self) -> &[T] {
        unsafe {
            let ptr: *const T = self.as_ptr();
            let len = self.len();
            let elm_size = std::mem::size_of::<T>();
            if !ptr.is_null() && len > elm_size {
                let n = len / elm_size;
                std::slice::from_raw_parts(ptr, n)
            } else {
                &[]
            }
        }
    }

    /// 返回指定类型的映射到用户空间的可读写数组切片。
    pub fn as_mut_slice<T>(&mut self) -> &mut [T] {
        unsafe {
            let ptr: *mut T = self.as_mut_ptr();
            let len = self.len();
            let elm_size = std::mem::size_of::<T>();
            if !ptr.is_null() && len > 0 {
                let n = len / elm_size;
                std::slice::from_raw_parts_mut(ptr, n)
            } else {
                &mut []
            }
        }
    }

    /// 当未分配到内存时返回 false。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 返回有效的字节长度。
    pub fn len(&self) -> usize {
        (self.bo.size - self.bo.offset) as usize
    }

    /// 返回用于 DMA 操作的文件描述符。
    pub fn dma_fd(&self) -> Result<i32, io::Error> {
        let mut fd: i32 = -1;
        unsafe {
            match ffi::c_RkRgaGetBufferFd(&self.bo as *const ffi::bo_t as *mut ffi::bo_t, &mut fd) {
                0 => Ok(fd),
                err => Err(io::Error::from_raw_os_error(err)),
            }
        }
    }

    /// 映射对象到用户空间。
    pub fn map(&mut self) -> Result<(), io::Error> {
        if !self.mapped {
            unsafe {
                match ffi::c_RkRgaGetMmap(&mut self.bo) {
                    0 => {
                        self.mapped = true;
                        Ok(())
                    }
                    err => Err(io::Error::from_raw_os_error(err)),
                }
            }
        } else {
            Ok(())
        }
    }

    /// 解除对象到用户空间的映射。
    pub fn unmap(&mut self) -> Result<(), io::Error> {
        if self.mapped {
            unsafe {
                match ffi::c_RkRgaUnmap(&mut self.bo) {
                    0 => {
                        self.mapped = false;
                        Ok(())
                    }
                    err => Err(io::Error::from_raw_os_error(err)),
                }
            }
        } else {
            Ok(())
        }
    }
}

impl Drop for RgaBuffer {
    fn drop(&mut self) {
        unsafe {
            if self.mapped {
                ffi::c_RkRgaUnmap(&mut self.bo);
                self.mapped = false;
            }
            ffi::c_RkRgaFree(&mut self.bo);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RgaPixelFormat, RgaRectBuilder};

    #[test]
    fn test_buffer_alloc() {
        let rga = Arc::new(Rga::new().unwrap());
        let bpps = &[8, 10, 12, 16, 18, 24, 30, 32];
        for xy in (16..=3840).step_by(128) {
            for bpp in bpps {
                let bo = RgaBuffer::new(Arc::clone(&rga), xy, xy, *bpp);
                assert!(bo.is_ok());
                let rect = RgaRectBuilder::new()
                    .size(xy, xy)
                    .format(RgaPixelFormat::Rgba8888)
                    .build();
                let bo = RgaBuffer::with_rect(Arc::clone(&rga), &rect);
                assert!(bo.is_ok());
            }
        }
    }

    #[test]
    fn test_buffer_mmap() {
        let rga = Arc::new(Rga::new().unwrap());
        let bo = RgaBuffer::new(Arc::clone(&rga), 3840, 2160, 32);
        assert!(bo.is_ok());
        if let Ok(mut bo) = bo {
            assert_eq!(unsafe { bo.ptr() }, std::ptr::null_mut());
            assert_eq!(bo.pitch(), 3840 * 4);
            assert_eq!(bo.size(), 3840 * 2160 * 4);
            assert!(bo.map().is_ok());
            assert!(bo.unmap().is_ok());
        }
    }
}
