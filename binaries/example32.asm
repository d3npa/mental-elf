BITS 32
GLOBAL _start
SECTION .text

_start:
    jmp short init

main:
    mov eax, 0x4
    mov ebx, 0x1
    pop ecx
    mov edx, 14
    int 0x80
    jmp short finish

init:
    call main
    db "Hello, world!", 0xa

finish:
    mov eax, 0x1
    xor ebx, ebx
    int 0x80
