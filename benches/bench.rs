#![feature(test)]

extern crate test;
use wasm_playground::Universe;

#[bench]
fn universe_ticks(bencher: &mut test::Bencher) {
    let mut universe = Universe::new_random(128, 128);

    bencher.iter(|| {
        universe.tick();
    });
}
