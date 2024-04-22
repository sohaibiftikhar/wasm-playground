//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
// use wasm_bindgen_test::*;
extern crate wasm_playground;
use wasm_playground::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
fn input_spaceship() -> Universe {
    let index = |row, col| Universe::index(6, row, col);
    let universe = Universe::new_with_cells(
        6,
        6,
        &[
            index(1, 2),
            index(2, 3),
            index(3, 1),
            index(3, 2),
            index(3, 3),
        ],
    );
    universe
}

#[cfg(test)]
fn expected_spaceship() -> Universe {
    let index = |row, col| Universe::index(6, row, col);
    let universe = Universe::new_with_cells(
        6,
        6,
        &[
            index(2, 1),
            index(2, 3),
            index(3, 2),
            index(3, 3),
            index(4, 2),
        ],
    );
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut input = input_spaceship();
    let expected = expected_spaceship();
    input.tick();
    assert_eq!(&input, &expected);
}
