#[wasm_import_module = "cwa"]
extern "C" {
    pub fn log_write(level: i32, text_ptr: *const u8, text_len: usize);
    pub fn env_get(
        key_ptr: *const u8, key_len: usize,
        value_buf_ptr: *mut u8, value_buf_len: usize
    ) -> i32;
    pub fn runtime_spec_major() -> i32;
    pub fn runtime_spec_minor() -> i32;
    pub fn runtime_name(out_ptr: *mut u8, out_len: usize) -> i32;
}
