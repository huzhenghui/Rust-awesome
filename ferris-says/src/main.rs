use ferris_says::say;
use std::io::stdout;
use std::io::BufWriter;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello from 胡争辉!");
    let width = message.as_bytes().len();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
