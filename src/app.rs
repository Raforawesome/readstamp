use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
	cx.render(
		rsx!(
			style { include_str!("./app.css") }
			h1 { "Hello, world!" }
		)
	)
}