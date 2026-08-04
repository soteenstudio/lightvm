.global main
.section .rodata
.section .rodata
nl_char:    .ascii "\n"
num_16:     .ascii "16"
obj_str:    .ascii "[Obj]"
arr_str:    .ascii "[Arr]"
cls_str:    .ascii "\033[H\033[J"
.text
main:
    // Allocate local variables stack space: 1 vars
    sub sp, sp, #16
    // ValIdx(0)
    ldr x9, [sp]
    str x9, [sp, #0]
    // PushInt32(1688888)
    movz x9, #1688888
    sub sp, sp, #16
    str x9, [sp]
    // SetIdx(0)
    ldr x9, [sp]
    str x9, [sp, #0]
    // GetIdx(0)
    ldr x9, [sp, #0]
    sub sp, sp, #16
    str x9, [sp]
    // Println (Runtime Mock)
    ldr x0, [sp]
    bl lightvm_println
    add sp, sp, #16
    ret
