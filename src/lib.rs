#[rustfmt::skip]
#[no_mangle]
pub static cargo_c_nasm_repro_table: &[u16; 32] = &[0; 32];

extern {
    fn cargo_c_nasm_repro_fn();
}

#[no_mangle]
pub unsafe extern fn repro() {
    cargo_c_nasm_repro_fn();
}
