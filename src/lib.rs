mod pairs;

use pairs::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
	static PAIRS: RefCell<Pairs> = RefCell::new(Pairs::new(4, 4));
}


#[wasm_bindgen(js_name = greet)]
pub fn greet() -> String {
	format!("Hallo")
}


#[wasm_bindgen(js_name = getState)]
pub fn get_pairs() -> String {
	PAIRS.with(|pairs| {
		pairs.borrow().to_string()
	})
}


#[wasm_bindgen(js_name = openField)]
pub fn open_field(y: usize, x: usize) -> bool {
	PAIRS.with(|pairs| {
		pairs.borrow_mut().open(x, y)
	})
}


#[wasm_bindgen(js_name = closeAll)]
pub fn close_all() {
	PAIRS.with(|pairs| {
		pairs.borrow_mut().close_all();
	})
}