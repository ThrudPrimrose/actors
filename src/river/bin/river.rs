use actors::actor::Acts;
use actors;

#[path = "../printer.rs"]
pub mod printer;

fn main() {
	let ap = printer::Printer {};
	ap.act();
	println!("Hello, this is river!");
}
