use super::ffi;

/// 一个描述 RGA 像素格式的枚举。
#[derive(Copy, Clone, Debug)]
pub enum RgaPixelFormat {
    Rgba8888,      // RK_FORMAT_RGBA_8888 = 0,
    Rgbx8888,      // RK_FORMAT_RGBX_8888 = 256,
    Rgb888,        // RK_FORMAT_RGB_888 = 512,
    Bgra8888,      // RK_FORMAT_BGRA_8888 = 768,
    Rgb565,        // RK_FORMAT_RGB_565 = 1024,
    Rgba5551,      // RK_FORMAT_RGBA_5551 = 1280,
    Rgba4444,      // RK_FORMAT_RGBA_4444 = 1536,
    Bgr888,        // RK_FORMAT_BGR_888 = 1792,
    YCbCr422sp,    // RK_FORMAT_YCbCr_422_SP = 2048,
    YCbCr422p,     // RK_FORMAT_YCbCr_422_P = 2304,
    YCbCr420sp,    // RK_FORMAT_YCbCr_420_SP = 2560,
    YCbCr420p,     // RK_FORMAT_YCbCr_420_P = 2816,
    YCrCb422sp,    // RK_FORMAT_YCrCb_422_SP = 3072,
    YCrCb422p,     // RK_FORMAT_YCrCb_422_P = 3328,
    YCrCb420sp,    // RK_FORMAT_YCrCb_420_SP = 3584,
    YCrCb420p,     // RK_FORMAT_YCrCb_420_P = 3840,
    Bpp1,          // RK_FORMAT_BPP1 = 4096,
    Bpp2,          // RK_FORMAT_BPP2 = 4352,
    Bpp4,          // RK_FORMAT_BPP4 = 4608,
    Bpp8,          // RK_FORMAT_BPP8 = 4864,
    Y4,            // RK_FORMAT_Y4 = 5120,
    YCbCr400,      // RK_FORMAT_YCbCr_400 = 5376,
    Bgrx8888,      // RK_FORMAT_BGRX_8888 = 5632,
    Yvyu422,       // RK_FORMAT_YVYU_422 = 6144,
    Yvyu420,       // RK_FORMAT_YVYU_420 = 6400,
    Vyuy422,       // RK_FORMAT_VYUY_422 = 6656,
    Vyuy420,       // RK_FORMAT_VYUY_420 = 6912,
    Yuyv422,       // RK_FORMAT_YUYV_422 = 7168,
    Yuyv420,       // RK_FORMAT_YUYV_420 = 7424,
    Uyvy422,       // RK_FORMAT_UYVY_422 = 7680,
    Uyvy420,       // RK_FORMAT_UYVY_420 = 7936,
    YCbCr420sp10b, // RK_FORMAT_YCbCr_420_SP_10B = 8192,
    YCrCb420sp10b, // RK_FORMAT_YCrCb_420_SP_10B = 8448,
    YCbCr422sp10b, // RK_FORMAT_YCbCr_422_10b_SP = 8704,
    YCrCb422sp10b, // RK_FORMAT_YCrCb_422_10b_SP = 8960,
    #[cfg(feature = "v1_2_5")]
    Bgr565, // RK_FORMAT_BGR_565 = 9216,
    #[cfg(feature = "v1_2_5")]
    Bgra5551, // RK_FORMAT_BGRA_5551 = 9472,
    #[cfg(feature = "v1_2_5")]
    Bgra4444, // RK_FORMAT_BGRA_4444 = 9728,
    #[cfg(feature = "v1_3_0")]
    Argb8888, // RK_FORMAT_ARGB_8888 = 10240,
    #[cfg(feature = "v1_3_0")]
    Xrgb8888, // RK_FORMAT_XRGB_8888 = 10496,
    #[cfg(feature = "v1_3_0")]
    Argb5551, // RK_FORMAT_ARGB_5551 = 10752,
    #[cfg(feature = "v1_3_0")]
    Argb4444, // RK_FORMAT_ARGB_4444 = 11008,
    #[cfg(feature = "v1_3_0")]
    Abgr8888, // RK_FORMAT_ABGR_8888 = 11264,
    #[cfg(feature = "v1_3_0")]
    Xbgr8888, // RK_FORMAT_XBGR_8888 = 11520,
    #[cfg(feature = "v1_3_0")]
    Abgr5551, // RK_FORMAT_ABGR_5551 = 11776,
    #[cfg(feature = "v1_3_0")]
    Abgr4444, // RK_FORMAT_ABGR_4444 = 12032,
    #[cfg(feature = "v1_7_2")]
    Rgba2Bpp, // RK_FORMAT_RGBA2BPP = 12288,
    Unknown,       // RK_FORMAT_UNKNOWN = 65536,
}

impl RgaPixelFormat {
    pub fn bits_per_pixel(&self) -> usize {
        use RgaPixelFormat::*;
        match self {
            Rgba8888 => 32,
            Rgbx8888 => 32,
            Rgb888 => 24,
            Bgra8888 => 32,
            Rgb565 => 16,
            Rgba5551 => 16,
            Rgba4444 => 16,
            Bgr888 => 24,
            YCbCr422sp => 16,
            YCbCr422p => 16,
            YCbCr420sp => 12,
            YCbCr420p => 12,
            YCrCb422sp => 16,
            YCrCb422p => 16,
            YCrCb420sp => 12,
            YCrCb420p => 12,
            Bpp1 => 1,
            Bpp2 => 2,
            Bpp4 => 4,
            Bpp8 => 8,
            Y4 => 4,
            YCbCr400 => 8,
            Bgrx8888 => 32,
            Yvyu422 => 16,
            Yvyu420 => 12,
            Vyuy422 => 16,
            Vyuy420 => 12,
            Yuyv422 => 16,
            Yuyv420 => 12,
            Uyvy422 => 16,
            Uyvy420 => 12,
            YCbCr420sp10b => 15,
            YCrCb420sp10b => 15,
            YCbCr422sp10b => 20,
            YCrCb422sp10b => 20,
            #[cfg(feature = "v1_2_5")]
            Bgr565 => 16,
            #[cfg(feature = "v1_2_5")]
            Bgra5551 => 16,
            #[cfg(feature = "v1_2_5")]
            Bgra4444 => 16,
            #[cfg(feature = "v1_3_0")]
            Argb8888 => 32,
            #[cfg(feature = "v1_3_0")]
            Xrgb8888 => 32,
            #[cfg(feature = "v1_3_0")]
            Argb5551 => 16,
            #[cfg(feature = "v1_3_0")]
            Argb4444 => 16,
            #[cfg(feature = "v1_3_0")]
            Abgr8888 => 32,
            #[cfg(feature = "v1_3_0")]
            Xbgr8888 => 32,
            #[cfg(feature = "v1_3_0")]
            Abgr5551 => 16,
            #[cfg(feature = "v1_3_0")]
            Abgr4444 => 16,
            #[cfg(feature = "v1_7_2")]
            Rgba2Bpp => 32,
            Unknown => 32,
        }
    }
}

impl From<ffi::RgaSURF_FORMAT> for RgaPixelFormat {
    fn from(val: ffi::RgaSURF_FORMAT) -> Self {
        use ffi::RgaSURF_FORMAT::*;
        use RgaPixelFormat::*;
        match val {
            RK_FORMAT_RGBA_8888 => Rgba8888,
            RK_FORMAT_RGBX_8888 => Rgbx8888,
            RK_FORMAT_RGB_888 => Rgb888,
            RK_FORMAT_BGRA_8888 => Bgra8888,
            RK_FORMAT_RGB_565 => Rgb565,
            RK_FORMAT_RGBA_5551 => Rgba5551,
            RK_FORMAT_RGBA_4444 => Rgba4444,
            RK_FORMAT_BGR_888 => Bgr888,
            RK_FORMAT_YCbCr_422_SP => YCbCr422sp,
            RK_FORMAT_YCbCr_422_P => YCbCr422p,
            RK_FORMAT_YCbCr_420_SP => YCbCr420sp,
            RK_FORMAT_YCbCr_420_P => YCbCr420p,
            RK_FORMAT_YCrCb_422_SP => YCrCb422sp,
            RK_FORMAT_YCrCb_422_P => YCrCb422p,
            RK_FORMAT_YCrCb_420_SP => YCrCb420sp,
            RK_FORMAT_YCrCb_420_P => YCrCb420p,
            RK_FORMAT_BPP1 => Bpp1,
            RK_FORMAT_BPP2 => Bpp2,
            RK_FORMAT_BPP4 => Bpp4,
            RK_FORMAT_BPP8 => Bpp8,
            RK_FORMAT_Y4 => Y4,
            RK_FORMAT_YCbCr_400 => YCbCr400,
            RK_FORMAT_BGRX_8888 => Bgrx8888,
            RK_FORMAT_YVYU_422 => Yvyu422,
            RK_FORMAT_YVYU_420 => Yvyu420,
            RK_FORMAT_VYUY_422 => Vyuy422,
            RK_FORMAT_VYUY_420 => Vyuy420,
            RK_FORMAT_YUYV_422 => Yuyv422,
            RK_FORMAT_YUYV_420 => Yuyv420,
            RK_FORMAT_UYVY_422 => Uyvy422,
            RK_FORMAT_UYVY_420 => Uyvy420,
            RK_FORMAT_YCbCr_420_SP_10B => YCbCr420sp10b,
            RK_FORMAT_YCrCb_420_SP_10B => YCrCb420sp10b,
            #[cfg(feature = "v1_7_2")]
            RK_FORMAT_YCbCr_422_SP_10B => YCbCr422sp10b,
            #[cfg(feature = "v1_7_2")]
            RK_FORMAT_YCrCb_422_SP_10B => YCrCb422sp10b,
            #[cfg(not(feature = "v1_7_2"))]
            RK_FORMAT_YCbCr_422_10b_SP => YCbCr422sp10b,
            #[cfg(not(feature = "v1_7_2"))]
            RK_FORMAT_YCrCb_422_10b_SP => YCrCb422sp10b,
            #[cfg(feature = "v1_2_5")]
            RK_FORMAT_BGR_565 => Bgr565,
            #[cfg(feature = "v1_2_5")]
            RK_FORMAT_BGRA_5551 => Bgra5551,
            #[cfg(feature = "v1_2_5")]
            RK_FORMAT_BGRA_4444 => Bgra4444,
            #[cfg(feature = "v1_3_0")]
            RK_FORMAT_ARGB_8888 => Argb8888,
            #[cfg(feature = "v1_3_0")]
            RK_FORMAT_XRGB_8888 => Xrgb8888,
            #[cfg(feature = "v1_3_0")]
            RK_FORMAT_ARGB_5551 => Argb5551,
            #[cfg(feature = "v1_3_0")]
            RK_FORMAT_ARGB_4444 => Argb4444,
            #[cfg(feature = "v1_3_0")]
            RK_FORMAT_ABGR_8888 => Abgr8888,
            #[cfg(feature = "v1_3_0")]
            RK_FORMAT_XBGR_8888 => Xbgr8888,
            #[cfg(feature = "v1_3_0")]
            RK_FORMAT_ABGR_5551 => Abgr5551,
            #[cfg(feature = "v1_3_0")]
            RK_FORMAT_ABGR_4444 => Abgr4444,
            #[cfg(feature = "v1_7_2")]
            RK_FORMAT_RGBA2BPP => Rgba2Bpp,
            RK_FORMAT_UNKNOWN => Unknown,
        }
    }
}

impl From<RgaPixelFormat> for ffi::RgaSURF_FORMAT {
    fn from(val: RgaPixelFormat) -> Self {
        use ffi::RgaSURF_FORMAT::*;
        use RgaPixelFormat::*;
        match val {
            Rgba8888 => RK_FORMAT_RGBA_8888,
            Rgbx8888 => RK_FORMAT_RGBX_8888,
            Rgb888 => RK_FORMAT_RGB_888,
            Bgra8888 => RK_FORMAT_BGRA_8888,
            Rgb565 => RK_FORMAT_RGB_565,
            Rgba5551 => RK_FORMAT_RGBA_5551,
            Rgba4444 => RK_FORMAT_RGBA_4444,
            Bgr888 => RK_FORMAT_BGR_888,
            YCbCr422sp => RK_FORMAT_YCbCr_422_SP,
            YCbCr422p => RK_FORMAT_YCbCr_422_P,
            YCbCr420sp => RK_FORMAT_YCbCr_420_SP,
            YCbCr420p => RK_FORMAT_YCbCr_420_P,
            YCrCb422sp => RK_FORMAT_YCrCb_422_SP,
            YCrCb422p => RK_FORMAT_YCrCb_422_P,
            YCrCb420sp => RK_FORMAT_YCrCb_420_SP,
            YCrCb420p => RK_FORMAT_YCrCb_420_P,
            Bpp1 => RK_FORMAT_BPP1,
            Bpp2 => RK_FORMAT_BPP2,
            Bpp4 => RK_FORMAT_BPP4,
            Bpp8 => RK_FORMAT_BPP8,
            Y4 => RK_FORMAT_Y4,
            YCbCr400 => RK_FORMAT_YCbCr_400,
            Bgrx8888 => RK_FORMAT_BGRX_8888,
            Yvyu422 => RK_FORMAT_YVYU_422,
            Yvyu420 => RK_FORMAT_YVYU_420,
            Vyuy422 => RK_FORMAT_VYUY_422,
            Vyuy420 => RK_FORMAT_VYUY_420,
            Yuyv422 => RK_FORMAT_YUYV_422,
            Yuyv420 => RK_FORMAT_YUYV_420,
            Uyvy422 => RK_FORMAT_UYVY_422,
            Uyvy420 => RK_FORMAT_UYVY_420,
            YCbCr420sp10b => RK_FORMAT_YCbCr_420_SP_10B,
            YCrCb420sp10b => RK_FORMAT_YCrCb_420_SP_10B,
            #[cfg(feature = "v1_7_2")]
            YCbCr422sp10b => RK_FORMAT_YCbCr_422_SP_10B,
            #[cfg(not(feature = "v1_7_2"))]
            YCbCr422sp10b => RK_FORMAT_YCbCr_422_10b_SP,
            #[cfg(feature = "v1_7_2")]
            YCrCb422sp10b => RK_FORMAT_YCrCb_422_SP_10B,
            #[cfg(not(feature = "v1_7_2"))]
            YCrCb422sp10b => RK_FORMAT_YCrCb_422_10b_SP,
            #[cfg(feature = "v1_2_5")]
            Bgr565 => RK_FORMAT_BGR_565,
            #[cfg(feature = "v1_2_5")]
            Bgra5551 => RK_FORMAT_BGRA_5551,
            #[cfg(feature = "v1_2_5")]
            Bgra4444 => RK_FORMAT_BGRA_4444,
            #[cfg(feature = "v1_3_0")]
            Argb8888 => RK_FORMAT_ARGB_8888,
            #[cfg(feature = "v1_3_0")]
            Xrgb8888 => RK_FORMAT_XRGB_8888,
            #[cfg(feature = "v1_3_0")]
            Argb5551 => RK_FORMAT_ARGB_5551,
            #[cfg(feature = "v1_3_0")]
            Argb4444 => RK_FORMAT_ARGB_4444,
            #[cfg(feature = "v1_3_0")]
            Abgr8888 => RK_FORMAT_ABGR_8888,
            #[cfg(feature = "v1_3_0")]
            Xbgr8888 => RK_FORMAT_XBGR_8888,
            #[cfg(feature = "v1_3_0")]
            Abgr5551 => RK_FORMAT_ABGR_5551,
            #[cfg(feature = "v1_3_0")]
            Abgr4444 => RK_FORMAT_ABGR_4444,
            #[cfg(feature = "v1_7_2")]
            Rgba2Bpp => RK_FORMAT_RGBA2BPP,
            Unknown => RK_FORMAT_UNKNOWN,
        }
    }
}
