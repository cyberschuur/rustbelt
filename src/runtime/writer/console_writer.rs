use super::Writer;

pub struct ConsoleWriter {}

impl Writer for ConsoleWriter {
    fn write_line(line: String) -> () {
        println!("{}", line)
    }
}
