Map vs Manual [Assembly from godbolt](https://godbolt.org/z/1vsjos3Eb)

```rust
pub fn stuff(arr: &mut [u32], a: usize, b: usize) -> [&mut u32; 2] {
    [a, b].map(|i| unsafe { &mut *std::ptr::addr_of_mut!(arr[i]) })
}
```

```asm
core::ptr::drop_in_place<core::array::Guard<&mut u32,2_usize>>:
        ret

example::stuff:
        push    r14
        push    rbx
        sub     rsp, 24
        cmp     rdx, rcx
        jbe     .LBB1_1
        lea     rax, [rsi + 4*rcx]
        mov     qword ptr [rsp + 8], rax
        cmp     rdx, r8
        jbe     .LBB1_5
        lea     rax, [rsi + 4*r8]
        mov     qword ptr [rsp + 16], rax
        mov     rax, qword ptr [rsp + 8]
        mov     qword ptr [rdi], rax
        mov     rax, qword ptr [rsp + 16]
        mov     qword ptr [rdi + 8], rax
        mov     rax, rdi
        add     rsp, 24
        pop     rbx
        pop     r14
        ret
.LBB1_1:
        xor     r14d, r14d
        jmp     .LBB1_2
.LBB1_5:
        mov     r14d, 1
        mov     rcx, r8
.LBB1_2:
        lea     rax, [rip + .L__unnamed_1]
        mov     rdi, rcx
        mov     rsi, rdx
        mov     rdx, rax
        call    qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        ud2
        mov     rbx, rax
        lea     rsi, [rsp + 8]
        mov     rdi, r14
        call    core::ptr::drop_in_place<core::array::Guard<&mut u32,2_usize>>
        mov     rdi, rbx
        call    _Unwind_Resume@PLT
        ud2

.L__unnamed_2:
        .ascii  "/app/example.rs"

.L__unnamed_1:
        .quad   .L__unnamed_2
        .asciz  "\017\000\000\000\000\000\000\000\002\000\000\000:\000\000"

DW.ref.rust_eh_personality:
        .quad   rust_eh_personality
```

```rust
pub fn stuff(arr: &mut [u32], a: usize, b: usize) -> [&mut u32; 2] {
    unsafe {
        [
            &mut *std::ptr::addr_of_mut!(arr[a]),
            &mut *std::ptr::addr_of_mut!(arr[b]),
        ]
    }
}
```

```asm
example::stuff:
        push    rax
        cmp     rcx, rdx
        jae     .LBB0_3
        cmp     r8, rdx
        jae     .LBB0_5
        lea     rax, [rsi + 4*rcx]
        lea     rcx, [rsi + 4*r8]
        mov     qword ptr [rdi], rax
        mov     qword ptr [rdi + 8], rcx
        mov     rax, rdi
        pop     rcx
        ret
.LBB0_3:
        lea     rax, [rip + .L__unnamed_1]
        mov     rdi, rcx
        jmp     .LBB0_4
.LBB0_5:
        lea     rax, [rip + .L__unnamed_2]
        mov     rdi, r8
.LBB0_4:
        mov     rsi, rdx
        mov     rdx, rax
        call    qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
        ud2

.L__unnamed_3:
        .ascii  "/app/example.rs"

.L__unnamed_1:
        .quad   .L__unnamed_3
        .asciz  "\017\000\000\000\000\000\000\000\b\000\000\000*\000\000"

.L__unnamed_2:
        .quad   .L__unnamed_3
        .asciz  "\017\000\000\000\000\000\000\000\t\000\000\000*\000\000"
```
