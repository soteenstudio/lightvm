.global main
.type main, %function
.section .rodata
nl_char:    .ascii "\n"
num_16:     .ascii "16"
obj_str:    .ascii "[Obj]"
arr_str:    .ascii "[Arr]"
cls_str:    .ascii "\033[H\033[J"
.text
main:
    // Establish stack frame
    sub sp, sp, #16
    str x19, [sp]
    mov x19, sp
    add x19, x19, #16
    // Allocate local frame: 1 vars + 0 InitStack slots
    sub sp, sp, #16
    // ValIdx(0)
    ldr x9, [sp]
    str x9, [x19, #-16]
    // PushBool(true)
    movz x9, #1
    sub sp, sp, #16
    str x9, [sp]
    // SetIdx(0)
    ldr x9, [sp]
    str x9, [x19, #-16]
    // GetIdx(0)
    ldr x9, [x19, #-16]
    sub sp, sp, #16
    str x9, [sp]
    // Println
    ldr x0, [sp]
    bl lightvm_println
    add sp, sp, #16
    // Restore stack pointer from frame base
    ldr x9, [x19, #-16]
    mov sp, x19
    mov x19, x9
    ret
