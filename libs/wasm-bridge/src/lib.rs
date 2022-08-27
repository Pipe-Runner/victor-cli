use wasm_bindgen::prelude::*;

// Externs let us tell rust that the functions defined here are actually defined
// elsewhere
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, World!");
}