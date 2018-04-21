use raw;

#[repr(i32)]
pub enum Level {
    Error = 1,
    Warning = 3,
    Info = 6
}

pub fn write(level: Level, text: &str) {
    let text = text.as_bytes();

    unsafe {
        raw::log_write(
            level as i32,
            slice_raw_ptr_or_null!(text),
            text.len()
        );
    }
}
