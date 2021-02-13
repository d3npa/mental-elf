all:
	nasm -f elf64 -o example64.o example64.asm
	ld -m elf_x86_64 -o example64 example64.o
	nasm -f elf32 -o example32.o example32.asm
	ld -m elf_i386 -o example32 example32.o

clean:
	rm example64 example64.o
	rm example32 example32.o
