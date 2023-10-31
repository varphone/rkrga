#![feature(test)]
extern crate test;
use paste::paste;
use rkrga::{Rga, RgaBuffer, RgaInfoBuilder, RgaPixelFormat, RgaRectBuilder};
use std::sync::Arc;
use test::Bencher;

macro_rules! bench_csc {
    ($sw:expr,$sh:expr,$tw:expr,$th:expr,$bpp:expr) => {
        paste! {
                #[bench]
                fn [<bench_csc_ $sw _ $sh _ $tw _ $th _ $bpp>](b: &mut Bencher) {
                let rga = Arc::new(Rga::new().unwrap());
                let fmt = match $bpp {
                    8 => RgaPixelFormat::YCbCr400,
                    12 => RgaPixelFormat::YCbCr420sp,
                    16 => RgaPixelFormat::Rgb565,
                    24 => RgaPixelFormat::Rgb888,
                    32 => RgaPixelFormat::Rgba8888,
                    _ => todo!(),
                };
                let src_rect = RgaRectBuilder::new().size($sw, $sh).format(fmt).build();
                let dst_rect = RgaRectBuilder::new().size($tw, $th).format(RgaPixelFormat::Rgb888).build();
                let src_bo = RgaBuffer::with_rect_mapped(Arc::clone(&rga), &src_rect).unwrap();
                let dst_bo = RgaBuffer::with_rect_mapped(Arc::clone(&rga), &dst_rect).unwrap();
                let src_info = RgaInfoBuilder::new().bo(&src_bo).rect(&src_rect).build();
                let mut dst_info = RgaInfoBuilder::new().bo(&dst_bo).rect(&dst_rect).build();
                b.iter(|| {
                    rga.scale(&src_info, &mut dst_info).unwrap();
                });
            }
        }
    };
}

macro_rules! bench_csc_vir {
    ($sw:expr,$sh:expr,$tw:expr,$th:expr,$bpp:expr) => {
        paste! {
                #[bench]
                fn [<bench_csc_vir_ $sw _ $sh _ $tw _ $th _ $bpp>](b: &mut Bencher) {
                let rga = Arc::new(Rga::new().unwrap());
                let fmt = match $bpp {
                    8 => RgaPixelFormat::YCbCr400,
                    12 => RgaPixelFormat::YCbCr420sp,
                    16 => RgaPixelFormat::Rgb565,
                    24 => RgaPixelFormat::Rgb888,
                    32 => RgaPixelFormat::Rgba8888,
                    _ => todo!(),
                };
                let src_rect = RgaRectBuilder::new().size($sw, $sh).format(fmt).build();
                let dst_rect = RgaRectBuilder::new().size($tw, $th).format(RgaPixelFormat::Rgb888).build();
                // let src_bo = RgaBuffer::with_rect_mapped(Arc::clone(&rga), &src_rect).unwrap();
                let mut src_buf: Vec<u8> = Vec::with_capacity($sw * $sh * $bpp / 8);
                let dst_bo = RgaBuffer::with_rect_mapped(Arc::clone(&rga), &dst_rect).unwrap();
                let src_info = RgaInfoBuilder::new().vir_addr(src_buf.as_mut_ptr()).rect(&src_rect).build();
                let mut dst_info = RgaInfoBuilder::new().bo(&dst_bo).rect(&dst_rect).build();
                b.iter(|| {
                    rga.scale(&src_info, &mut dst_info).unwrap();
                });
            }
        }
    };
}

bench_csc!(320, 240, 1920, 1080, 8);
bench_csc!(320, 240, 1920, 1080, 12);
bench_csc!(320, 240, 1920, 1080, 16);
bench_csc!(320, 240, 1920, 1080, 24);
bench_csc!(320, 240, 1920, 1080, 32);

bench_csc!(320, 240, 3840, 2160, 8);
bench_csc!(320, 240, 3840, 2160, 12);
bench_csc!(320, 240, 3840, 2160, 16);
bench_csc!(320, 240, 3840, 2160, 24);
bench_csc!(320, 240, 3840, 2160, 32);

bench_csc!(1920, 1080, 320, 240, 8);
bench_csc!(1920, 1080, 320, 240, 12);
bench_csc!(1920, 1080, 320, 240, 16);
bench_csc!(1920, 1080, 320, 240, 24);
bench_csc!(1920, 1080, 320, 240, 32);

bench_csc!(1920, 1080, 416, 416, 8);
bench_csc!(1920, 1080, 416, 416, 12);
bench_csc!(1920, 1080, 416, 416, 16);
bench_csc!(1920, 1080, 416, 416, 24);
bench_csc!(1920, 1080, 416, 416, 32);

bench_csc!(1920, 1080, 3840, 2160, 8);
bench_csc!(1920, 1080, 3840, 2160, 12);
bench_csc!(1920, 1080, 3840, 2160, 16);
bench_csc!(1920, 1080, 3840, 2160, 24);
bench_csc!(1920, 1080, 3840, 2160, 32);

bench_csc!(3840, 2160, 320, 240, 8);
bench_csc!(3840, 2160, 320, 240, 12);
bench_csc!(3840, 2160, 320, 240, 16);
bench_csc!(3840, 2160, 320, 240, 24);
bench_csc!(3840, 2160, 320, 240, 32);

bench_csc!(3840, 2160, 416, 416, 8);
bench_csc!(3840, 2160, 416, 416, 12);
bench_csc!(3840, 2160, 416, 416, 16);
bench_csc!(3840, 2160, 416, 416, 24);
bench_csc!(3840, 2160, 416, 416, 32);

bench_csc!(3840, 2160, 1920, 1080, 8);
bench_csc!(3840, 2160, 1920, 1080, 12);
bench_csc!(3840, 2160, 1920, 1080, 16);
bench_csc!(3840, 2160, 1920, 1080, 24);
bench_csc!(3840, 2160, 1920, 1080, 32);

bench_csc_vir!(320, 240, 1920, 1080, 8);
bench_csc_vir!(320, 240, 1920, 1080, 12);
bench_csc_vir!(320, 240, 1920, 1080, 16);
bench_csc_vir!(320, 240, 1920, 1080, 24);
bench_csc_vir!(320, 240, 1920, 1080, 32);

bench_csc_vir!(320, 240, 3840, 2160, 8);
bench_csc_vir!(320, 240, 3840, 2160, 12);
bench_csc_vir!(320, 240, 3840, 2160, 16);
bench_csc_vir!(320, 240, 3840, 2160, 24);
bench_csc_vir!(320, 240, 3840, 2160, 32);

bench_csc_vir!(1920, 1080, 320, 240, 8);
bench_csc_vir!(1920, 1080, 320, 240, 12);
bench_csc_vir!(1920, 1080, 320, 240, 16);
bench_csc_vir!(1920, 1080, 320, 240, 24);
bench_csc_vir!(1920, 1080, 320, 240, 32);

bench_csc_vir!(1920, 1080, 416, 416, 8);
bench_csc_vir!(1920, 1080, 416, 416, 12);
bench_csc_vir!(1920, 1080, 416, 416, 16);
bench_csc_vir!(1920, 1080, 416, 416, 24);
bench_csc_vir!(1920, 1080, 416, 416, 32);

bench_csc_vir!(1920, 1080, 3840, 2160, 8);
bench_csc_vir!(1920, 1080, 3840, 2160, 12);
bench_csc_vir!(1920, 1080, 3840, 2160, 16);
bench_csc_vir!(1920, 1080, 3840, 2160, 24);
bench_csc_vir!(1920, 1080, 3840, 2160, 32);

bench_csc_vir!(3840, 2160, 320, 240, 8);
bench_csc_vir!(3840, 2160, 320, 240, 12);
bench_csc_vir!(3840, 2160, 320, 240, 16);
bench_csc_vir!(3840, 2160, 320, 240, 24);
bench_csc_vir!(3840, 2160, 320, 240, 32);

bench_csc_vir!(3840, 2160, 416, 416, 8);
bench_csc_vir!(3840, 2160, 416, 416, 12);
bench_csc_vir!(3840, 2160, 416, 416, 16);
bench_csc_vir!(3840, 2160, 416, 416, 24);
bench_csc_vir!(3840, 2160, 416, 416, 32);

bench_csc_vir!(3840, 2160, 1920, 1080, 8);
bench_csc_vir!(3840, 2160, 1920, 1080, 12);
bench_csc_vir!(3840, 2160, 1920, 1080, 16);
bench_csc_vir!(3840, 2160, 1920, 1080, 24);
bench_csc_vir!(3840, 2160, 1920, 1080, 32);
