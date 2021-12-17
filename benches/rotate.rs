#![feature(test)]
extern crate test;
use paste::paste;
use rkrga::{Rga, RgaBuffer, RgaInfoBuilder, RgaPixelFormat, RgaRectBuilder, RgaTransform};
use std::sync::Arc;
use test::Bencher;

macro_rules! bench_rotate {
    ($w:expr,$h:expr,$bpp:expr) => {
        paste! {
                #[bench]
                fn [<bench_rotate_ $w _ $h _ $bpp>](b: &mut Bencher) {
                let rga = Arc::new(Rga::new().unwrap());
                let fmt = match $bpp {
                    8 => RgaPixelFormat::YCbCr400,
                    16 => RgaPixelFormat::Rgb565,
                    24 => RgaPixelFormat::Rgb888,
                    32 => RgaPixelFormat::Rgba8888,
                    _ => todo!(),
                };
                let src_rect = RgaRectBuilder::new().size($w, $h).format(fmt).build();
                let dst_rect = RgaRectBuilder::new().size($h, $w).format(fmt).build();
                let src_bo = RgaBuffer::with_rect_mapped(Arc::clone(&rga), &src_rect).unwrap();
                let dst_bo = RgaBuffer::with_rect_mapped(Arc::clone(&rga), &dst_rect).unwrap();
                let src_info = RgaInfoBuilder::new().bo(&src_bo).rect(&src_rect).build();
                let mut dst_info = RgaInfoBuilder::new().bo(&dst_bo).rect(&dst_rect).build();
                b.iter(|| {
                    rga.rotate(&src_info, &mut dst_info, RgaTransform::Rot90).unwrap();
                });
            }
        }
    };
}

bench_rotate!(320, 240, 8);
bench_rotate!(320, 240, 16);
bench_rotate!(320, 240, 24);
bench_rotate!(320, 240, 32);

bench_rotate!(640, 480, 8);
bench_rotate!(640, 640, 16);
bench_rotate!(640, 640, 24);
bench_rotate!(640, 640, 32);

bench_rotate!(1920, 1080, 8);
bench_rotate!(1920, 1080, 16);
bench_rotate!(1920, 1080, 24);
bench_rotate!(1920, 1080, 32);

bench_rotate!(3840, 2160, 8);
bench_rotate!(3840, 2160, 16);
bench_rotate!(3840, 2160, 24);
bench_rotate!(3840, 2160, 32);

bench_rotate!(480, 1080, 32);
bench_rotate!(960, 1080, 32);
