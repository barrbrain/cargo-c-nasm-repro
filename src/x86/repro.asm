%ifdef PREFIX
    %define mangle(x) _ %+ x
%else
    %define mangle(x) x
%endif

default rel
extern mangle(cargo_c_nasm_repro_table)

section .text
global mangle(cargo_c_nasm_repro_fn)

mangle(cargo_c_nasm_repro_fn):
lea R10, [mangle(cargo_c_nasm_repro_table)]
ret
