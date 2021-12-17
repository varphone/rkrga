#![feature(test)]
extern crate test;
use paste::paste;
use rkrga::Rga;
use std::sync::Arc;
use test::{black_box, Bencher};

macro_rules! bench_alloc {
    ($w:expr,$h:expr,$bpp:expr) => {
        paste! {
                #[bench]
                fn [<bench_alloc_ $w _ $h _ $bpp>](b: &mut Bencher) {
                let rga = Arc::new(Rga::new().unwrap());
                b.iter(|| {
                    let bo = rga.alloc_buffer($w, $h, $bpp);
                    assert_eq!(bo.is_ok(), true);
                    black_box(bo).unwrap();
                });
            }
        }
    };
}

macro_rules! bench_mmap {
    ($w:expr,$h:expr,$bpp:expr) => {
        paste! {
            #[bench]
            fn [<bench_mmap_ $w _ $h _ $bpp>](b: &mut Bencher) {
                let rga = Arc::new(Rga::new().unwrap());
                let mut bo = rga.alloc_buffer($w, $h, $bpp).unwrap();
                b.iter(|| {
                    assert_eq!(bo.map().is_ok(), true);
                    assert_eq!(bo.len(), $w * $h * $bpp / 8);
                    assert_eq!(bo.unmap().is_ok(), true);
                });
            }
        }
    };
}

bench_alloc!(320, 240, 8);
bench_alloc!(320, 240, 16);
bench_alloc!(320, 240, 24);
bench_alloc!(320, 240, 32);
bench_alloc!(1920, 1080, 8);
bench_alloc!(1920, 1080, 16);
bench_alloc!(1920, 1080, 24);
bench_alloc!(1920, 1080, 32);
bench_alloc!(3840, 2160, 8);
bench_alloc!(3840, 2160, 16);
bench_alloc!(3840, 2160, 24);
bench_alloc!(3840, 2160, 32);

bench_mmap!(320, 240, 8);
bench_mmap!(320, 240, 16);
bench_mmap!(320, 240, 24);
bench_mmap!(320, 240, 32);
bench_mmap!(1920, 1080, 8);
bench_mmap!(1920, 1080, 16);
bench_mmap!(1920, 1080, 24);
bench_mmap!(1920, 1080, 32);
bench_mmap!(3840, 2160, 8);
bench_mmap!(3840, 2160, 16);
bench_mmap!(3840, 2160, 24);
bench_mmap!(3840, 2160, 32);
