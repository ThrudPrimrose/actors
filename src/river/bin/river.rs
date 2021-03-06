use actors;

#[path = "../printer.rs"]
pub mod printer;

fn main() {
	let ap = actors::actor::Actor {
		core: printer::Printer {},
	};
	ap.act();
	println!("Hello, this is river!");
}
