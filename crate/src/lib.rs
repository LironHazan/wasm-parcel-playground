#[macro_use]
extern crate cfg_if;

extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::Element;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

pub trait Drawable {
    fn draw(&self, shape: Shape) -> Result<(), JsValue>;
}

pub struct ShapeDrawer {
    document: web_sys::Document,
    svg: Element
}

pub struct Circle {
    cx:  &'static str,
    cy:  &'static str,
    r:  &'static str,
}

pub enum Shape {
    Circle( Circle)
}

impl Circle {
    fn new(cx: &'static str,
           cy: &'static str,
           r: &'static str,
    ) -> Self {
        Self { cx, cy, r }
    }
}

impl ShapeDrawer {
    fn new(document: web_sys::Document, svg: Element) -> Self {
        Self { document, svg }
    }
}

impl Drawable for ShapeDrawer {
    fn draw(&self, shape: Shape) -> Result<(), JsValue> {
            if let Shape::Circle(circ) = shape {
                let circle = self.document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
                circle.set_attribute("cx", &circ.cx)?;
                circle.set_attribute("cy", &circ.cy)?;
                circle.set_attribute("r", &circ.r)?;
                circle.set_attribute("stroke", "#fff")?;
                circle.set_attribute("stroke-width", "10")?;
                circle.set_attribute("fill", "hotpink")?;
                self.document.body().expect("document expect to have have a body").append_child(&self.svg)?;
                self.svg.append_child(&circle)?;
            }
        Ok(())
    }
}

fn get_document() ->  web_sys::Document {
    let window = web_sys::window().expect("no global `window` exists");
    window.document().expect("should have a document on window")
}

fn awesome_wasm_drawing() -> Result<(), JsValue> {
    let document = get_document();

    let p = document.create_element("p")?;
    p.set_inner_html("Hello from Rust, WebAssembly, and Parcel!");
    document.body().expect("document expect to have have a body").append_child(&p)?;

    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    svg.set_attribute("width", "500")?;
    svg.set_attribute("hight", "500")?;

    let drawer = ShapeDrawer::new(document, svg);
    // Cool illustration
    let circles = vec![
        Circle::new("80", "80", "60"),
        Circle::new("55", "80", "60"),
        Circle::new("80", "80", "20")
    ];

    for c in circles {
        drawer.draw(Shape::Circle(c));
    }
    Ok(())
}
// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();
    console::log_1(&"Lets play with SVG".into());
    awesome_wasm_drawing();
    Ok(())
}


