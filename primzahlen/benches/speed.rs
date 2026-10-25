#![feature(test)]

extern crate test;

use test::{Bencher, black_box};

use primzahlen::ist_primzahl::*;


const MAX: u32 = 1_000;


fn loop_v1(max: u32) {
    for n in 1..max {
        let _x = black_box(v1(n));
    }
}

fn loop_v2(max: u32) {
    for n in 1..max {
        let _x = black_box(v2(n));
    }
}

fn loop_v3(max: u32) {
    for n in 1..max {
        let _x = black_box(v3(n));
    }
}

fn loop_v4(max: u32) {
    for n in 1..max {
        let _x = black_box(v4(n));
    }
}


#[bench]
fn bench_v1(b: &mut Bencher) {
    b.iter(|| loop_v1(black_box(MAX)));
}

#[bench]
fn bench_v2(b: &mut Bencher) {
    b.iter(|| loop_v2(black_box(MAX)))
}

#[bench]
fn bench_v3(b: &mut Bencher) {
    b.iter(|| loop_v3(black_box(MAX)))
}

#[bench]
fn bench_v4(b: &mut Bencher) {
    b.iter(|| loop_v4(black_box(MAX)))
}
