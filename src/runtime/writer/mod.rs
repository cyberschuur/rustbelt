pub mod console_writer;

pub trait Writer {
    fn write_line(line: String) -> ();
}
