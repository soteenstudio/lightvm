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

// ANSI Color Codes
#define COLOR_RESET   "\033[0m"
#define COLOR_INT     "\033[32m" // Hijau
#define COLOR_BOOL    "\033[35m" // Magenta / Ungu
#define COLOR_FLOAT   "\033[36m" // Cyan / Biru Muda
#define COLOR_STRING  "\033[33m" // Kuning

typedef struct {
  long long type_tag; // 0 = Int, 1 = Bool, 2 = Float, 3 = String, dst.
  long long value;    // Nilai aslinya atau pointer string di .rodata
} LightVMValue;

void lightvm_print(LightVMValue* val_ptr) {
  if (!val_ptr) return;

  if (val_ptr->type_tag == 1) {
    // Tag 1 adalah Boolean (Ungu)
    printf("%s%s%s", COLOR_BOOL, val_ptr->value ? "true" : "false", COLOR_RESET);
  } else if (val_ptr->type_tag == 2) {
    // Tag 2 adalah Float (Cyan)
    printf("%s%f%s", COLOR_FLOAT, *(double*)&val_ptr->value, COLOR_RESET);
  } else if (val_ptr->type_tag == 3) {
    // Tag 3 adalah String (Kuning)
    printf("%s%s%s", COLOR_STRING, (char*)val_ptr->value, COLOR_RESET);
  } else {
    // Tag 0 atau lainnya dicetak sebagai integer biasa (Hijau)
    printf("%s%lld%s", COLOR_INT, val_ptr->value, COLOR_RESET);
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
