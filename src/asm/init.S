#define EL0 0b00
#define EL1 0b01
#define EL2 0b10
#define EL3 0b11

.section .text.init

.global _start
_start:
    // read cpu affinity, start core 0, halt rest
    mrs     x1, MPIDR_EL1
    and     x1, x1, #3
    cbz     x1, setup

halt:
    // core affinity != 0, halt it
    wfe
    b       halt

setup:
    // store the desired EL1 stack pointer in x1
    adr     x1, _start

    // read the current exception level into x0 (ref: C5.2.1)
    mrs     x0, CurrentEL
    and     x0, x0, #0b1100
    lsr     x0, x0, #2

switch_to_el2:
    // switch to EL2 if we're in EL3. otherwise switch to EL1
    cmp     x0, 0b11
    bne     switch_to_el1

    // set-up SCR_EL3 (bits 0, 4, 5, 7, 8, 10) (A53: 4.3.42)
    mov     x2, #0x5b1
    msr     SCR_EL3, x2

    // set-up SPSR and PL switch! (bits 0, 3, 6, 7, 8, 9) (ref: C5.2.20)
    mov     x2, #0x3c9
    msr     SPSR_EL3, x2
    adr     x2, switch_to_el1
    msr     ELR_EL3, x2
    eret

switch_to_el1:
    // switch to EL1 if we're not already in EL1. otherwise continue with start
    cmp     x0, 0b01
    beq     set_stack

    // set the stack-pointer for EL1
    msr     SP_EL1, x1

    // enable CNTP for EL1/EL0 (ref: D7.5.2, D7.5.13)
    // NOTE: This doesn't actually enable the counter stream.
    mrs     x0, CNTHCTL_EL2
    orr     x0, x0, #0b11
    msr     CNTHCTL_EL2, x0
    msr     CNTVOFF_EL2, xzr

    // enable AArch64 in EL1 (A53: 4.3.36)
    mov     x0, #(1 << 31)      // Enable AArch64 for EL1
    orr     x0, x0, #(1 << 1)   // RES1 on A-53
    msr     HCR_EL2, x0
    mrs     x0, HCR_EL2

    // enable floating point and SVE (SIMD) (A53: 4.3.38, 4.3.34)
    msr     CPTR_EL2, xzr     // don't trap accessing SVE registers
    mrs     x0, CPACR_EL1
    orr     x0, x0, #(0b11 << 20)
    msr     CPACR_EL1, x0

    // Set SCTLR to known state (RES1: 11, 20, 22, 23, 28, 29) (A53: 4.3.30)
    mov     x2, #0x0800
    movk    x2, #0x30d0, lsl #16
    msr     SCTLR_EL1, x2

    // set up exception handlers
    // FIXME: load `_vectors` addr into appropriate register (guide: 10.4)
	adr    x0, _vectors
	msr    vbar_el1, x0


    // change execution level to EL1 (ref: C5.2.19)
    mov     x2, #0x3c5
    msr     SPSR_EL2, x2

	adr    x0, set_stack
	msr    elr_el2, x0

	eret

set_stack:
    // set the current stack pointer
    mov     sp, x1

zero_bss:
    // load the start address and number of bytes in BSS section
    ldr     x1, =__bss_start
    ldr     x2, =__bss_length

zero_bss_loop:
    // zero out the BSS section, 64-bits at a time
    cbz     x2, go_kmain
    str     xzr, [x1], #8
    sub     x2, x2, #8
    cbnz    x2, zero_bss_loop

go_kmain:
    // jump to kmain, which shouldn't return. halt if it does
    bl      main
    b       halt

context_save:
	sub	sp, sp, #16*33
	stp q0, q1, [sp, #16*2]
	stp q2, q3, [sp, #16*3]
	stp q4, q5, [sp, #16*4]
	stp q6, q7, [sp, #16*5]
	stp q8, q9, [sp, #16*6]
	stp q10, q11, [sp, #16*7]
	stp q12, q13, [sp, #16*8]
	stp q14, q15, [sp, #16*9]
	stp q16, q17, [sp, #16*10]
	stp q18, q19, [sp, #16*11]
	stp q20, q21, [sp, #16*12]
	stp q22, q23, [sp, #16*13]
	stp q24, q25, [sp, #16*14]
	stp q26, q27, [sp, #16*15]
	stp q28, q29, [sp, #16*16]
	stp q30, q31, [sp, #16*17]
	stp x1, x2, [sp, #16*18]
	stp x3, x4, [sp, #16*19]
	stp x5, x6, [sp, #16*20]
	stp x7, x8, [sp, #16*21]
	stp	x9, x10, [sp, #16*22]
	stp	x11, x12, [sp, #16*23]
	stp	x13, x14, [sp, #16*24]
	stp	x15, x16, [sp, #16*25]
	stp x17, x18, [sp, #16*26]
	stp x19, x20, [sp, #16*27]
	stp x21, x22, [sp, #16*28]
	stp x23, x24, [sp, #16*29]
	stp x25, x26, [sp, #16*30]
	stp x27, x28, [sp, #16*31]
	str x29, [sp, #16*32]

	mrs x1, elr_el1
	mrs x2, spsr_el1
	stp x1, x2, [sp, #16*0]
	mrs x1, sp_el0
	mrs x2, tpidr_el0
	stp x1, x2, [sp, #16*1]

	mrs    x1, esr_el1
	mov x2, sp
	sub sp, sp, #16*1
	str lr, [sp, #16*0]
	bl handle_exception
	ldr lr, [sp, #16*0]
	add sp, sp, #16*1

.global context_restore
context_restore:
	ldp x1, x2, [sp, #16*1]
	msr tpidr_el0, x2
	msr sp_el0, x1
	ldp x1, x2, [sp, #16*0]
	msr spsr_el1, x2
	msr elr_el1, x1

	ldr x29, [sp, #16*32]
	ldp x27, x28, [sp, #16*31]
	ldp x25, x26, [sp, #16*30]
	ldp x23, x24, [sp, #16*29]
	ldp x21, x22, [sp, #16*28]
	ldp x19, x20, [sp, #16*27]
	ldp x17, x18, [sp, #16*26]
	ldp x15, x16, [sp, #16*25]
	ldp x13, x14, [sp, #16*24]
	ldp x11, x12, [sp, #16*23]
	ldp x9, x10, [sp, #16*22]
	ldp x7, x8, [sp, #16*21]
	ldp x5, x6, [sp, #16*20]
	ldp x3, x4, [sp, #16*19]
	ldp x1, x2, [sp, #16*18]
	ldp q30, q31, [sp, #16*17]
	ldp q28, q29, [sp, #16*16]
	ldp q26, q27, [sp, #16*15]
	ldp q24, q25, [sp, #16*14]
	ldp q22, q23, [sp, #16*13]
	ldp q20, q21, [sp, #16*12]
	ldp q18, q19, [sp, #16*11]
	ldp q16, q17, [sp, #16*10]
	ldp q14, q15, [sp, #16*9]
	ldp q12, q13, [sp, #16*8]
	ldp q10, q11, [sp, #16*7]
	ldp q8, q9, [sp, #16*6]
	ldp q6, q7, [sp, #16*5]
	ldp q4, q5, [sp, #16*4]
	ldp q2, q3, [sp, #16*3]
	ldp q0, q1, [sp, #16*2]
	add	sp, sp, #16*33

    ret



.macro HANDLER source kind
	.align 7
	stp     lr, x0, [SP, #-16]!
	mov     x0, #\source
	movk    x0, #\kind, LSL #16
	bl      context_save
	ldp     lr, x0, [SP], #16
	eret
.endm


.align 11
_vectors:
    HANDLER 0 0
    HANDLER 0 1
    HANDLER 0 2
    HANDLER 0 3
    HANDLER 1 0
    HANDLER 1 1
    HANDLER 1 2
    HANDLER 1 3
    HANDLER 2 0
    HANDLER 2 1
    HANDLER 2 2
    HANDLER 2 3
    HANDLER 3 0
    HANDLER 3 1
    HANDLER 3 2
    HANDLER 3 3
