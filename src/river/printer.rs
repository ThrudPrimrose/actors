use actors;

pub struct Printer {}

impl actors::actor::Acts for Printer {
    fn act(&self) {
        println!("A very basic pritner actor");
    }
}
