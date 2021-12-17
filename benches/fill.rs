#![feature(test)]
extern crate test;
use paste::paste;
use rkrga::{Rga, RgaBuffer, RgaInfoBuilder, RgaPixelFormat, RgaRectBuilder};
use std::sync::Arc;
use test::Bencher;

macro_rules! bench_fill {
    ($w:expr,$h:expr,$bpp:expr) => {
        paste! {
                #[bench]
                fn [<bench_fill_ $w _ $h _ $bpp>](b: &mut Bencher) {
                let rga = Arc::new(Rga::new().unwrap());
                let fmt = match $bpp {
                    8 => RgaPixelFormat::YCbCr400,
                    16 => RgaPixelFormat::Rgb565,
                    24 => RgaPixelFormat::Rgb888,
                    32 => RgaPixelFormat::Rgba8888,
                    _ => todo!(),
                };
                let rect = RgaRectBuilder::new().size($w, $h).format(fmt).build();
                let bo = RgaBuffer::with_rect_mapped(Arc::clone(&rga), &rect).unwrap();
                let info = RgaInfoBuilder::new().bo(&bo).rect(&rect).build();
                b.iter(|| {
                    rga.fill(&info, 0xff33_77aa).unwrap();
                });
            }
        }
    };
}

macro_rules! bench_fill_vir {
    ($w:expr,$h:expr,$bpp:expr) => {
        paste! {
                #[bench]
                fn [<bench_fill_vir_ $w _ $h _ $bpp>](b: &mut Bencher) {
                let rga = Arc::new(Rga::new().unwrap());
                let fmt = match $bpp {
                    8 => RgaPixelFormat::YCbCr400,
                    16 => RgaPixelFormat::Rgb565,
                    24 => RgaPixelFormat::Rgb888,
                    32 => RgaPixelFormat::Rgba8888,
                    _ => todo!(),
                };
                let rect = RgaRectBuilder::new().size($w, $h).format(fmt).build();
                let mut buf: Vec<u8> = Vec::with_capacity($w * $h * $bpp / 8);
                let info = RgaInfoBuilder::new().vir_addr(buf.as_mut_ptr()).rect(&rect).build();
                b.iter(|| {
                    rga.fill(&info, 0xff33_77aa).unwrap();
                });
            }
        }
    };
}

bench_fill!(320, 240, 8);
bench_fill!(320, 240, 16);
bench_fill!(320, 240, 24);
bench_fill!(320, 240, 32);

bench_fill!(640, 480, 8);
bench_fill!(640, 640, 16);
bench_fill!(640, 640, 24);
bench_fill!(640, 640, 32);

bench_fill!(1920, 1080, 8);
bench_fill!(1920, 1080, 16);
bench_fill!(1920, 1080, 24);
bench_fill!(1920, 1080, 32);

bench_fill!(3840, 2160, 8);
bench_fill!(3840, 2160, 16);
bench_fill!(3840, 2160, 24);
bench_fill!(3840, 2160, 32);

bench_fill_vir!(320, 240, 8);
bench_fill_vir!(320, 240, 16);
bench_fill_vir!(320, 240, 24);
bench_fill_vir!(320, 240, 32);

bench_fill_vir!(640, 480, 8);
bench_fill_vir!(640, 640, 16);
bench_fill_vir!(640, 640, 24);
bench_fill_vir!(640, 640, 32);

bench_fill_vir!(1920, 1080, 8);
bench_fill_vir!(1920, 1080, 16);
bench_fill_vir!(1920, 1080, 24);
bench_fill_vir!(1920, 1080, 32);

bench_fill_vir!(3840, 2160, 8);
bench_fill_vir!(3840, 2160, 16);
bench_fill_vir!(3840, 2160, 24);
bench_fill_vir!(3840, 2160, 32);
