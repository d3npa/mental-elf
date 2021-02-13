BITS 64
GLOBAL _start
SECTION .text

_start:
    jmp short init

main:
    mov rax, 0x1
    mov rdi, rax
    pop rsi
    mov rdx, 14
    syscall
    jmp short finish

init:
    call main
    db "Hello, world!", 0xa

finish:
    mov rax, 0x3c
    xor rdi, rdi
    syscall