/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#include <stdio.h>

typedef struct {
  long long type_tag; // 0 = Int, 1 = Bool, 2 = Float, 3 = String, dst.
  long long value;    // Nilai aslinya atau pointer string di .rodata
} LightVMValue;

void lightvm_print(LightVMValue* val_ptr) {
  if (!val_ptr) return;

  if (val_ptr->type_tag == 1) {
    // Tag 1 adalah Boolean
    printf("%s", val_ptr->value ? "true" : "false");
  } else if (val_ptr->type_tag == 2) {
    // Tag 2 adalah Float (jika nilainya di-cast dari double/float representation)
    printf("%f", *(double*)&val_ptr->value);
  } else if (val_ptr->type_tag == 3) {
    // Tag 3 adalah String (value adalah pointer char*)
    printf("%s", (char*)val_ptr->value);
  } else {
    // Tag 0 atau lainnya dicetak sebagai integer biasa
    printf("%lld", val_ptr->value);
  }
}

void lightvm_println(LightVMValue* val_ptr) {
  lightvm_print(val_ptr);
  printf("\n");
}

void lightvm_stdout(LightVMValue* val_ptr) {
  lightvm_print(val_ptr);
}

void lightvm_stdoutln(LightVMValue* val_ptr) {
  lightvm_println(val_ptr);
}

void* lightvm_stdin(void* buf) {
  if (buf) {
    *(long long*)buf = 0;
    scanf("%lld", (long long*)buf);
  }
  return buf;
}

void lightvm_inspect_obj(void* ptr) {
  if (!ptr) {
    printf("{}");
    return;
  }
  printf("{ /* object ptr: %p */ }", ptr);
}

void lightvm_inspect_arr(void* ptr) {
  if (!ptr) {
    printf("[]");
    return;
  }
  printf("[ /* array ptr: %p */ ]", ptr);
}

void lightvm_clear_screen(void) {
  printf("\033[H\033[J");
}
