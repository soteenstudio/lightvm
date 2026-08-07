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

// These colors are provided to make the display on the CLI more readable.
// - Green -> Used for number colors (eg: integer, float, etc.)
// - Megenta or purple -> Used for boolean colors (eg: true and false)
// - Cyan or light blue -> Used for object and array colors
// - Yellow -> Used for string colors
#define COLOR_RESET   "\033[0m"
#define COLOR_INT     "\033[32m"
#define COLOR_BOOL    "\033[35m"
#define COLOR_OBJ   "\033[36m"
#define COLOR_STRING  "\033[33m"

typedef struct {
  long long type_tag;
  long long value;    // Original value or string pointer in .rodata
} LightVMValue;

void lightvm_print(LightVMValue* val_ptr) {
  if (!val_ptr) return;
  
  // 0 -> Tag for Fallback
  // 1 -> Tag for Boolean
  // 2 -> Tag for Number
  // 3 -> Tag for Object/Array
  // 4 -> Tag for String
  if (val_ptr->type_tag == 0) {
    printf("%s%lld%s", COLOR_INT, val_ptr->value, COLOR_RESET);
  } else if (val_ptr->type_tag == 1) {
    printf("%s%s%s", COLOR_BOOL, val_ptr->value ? "true" : "false", COLOR_RESET);
  } else if (val_ptr->type_tag == 2) {
    printf("%s%f%s", COLOR_INT, *(double*)&val_ptr->value, COLOR_RESET);
  } else if (val_ptr->type_tag == 3) {
    printf("%s%s%s", COLOR_OBJ, (char*)val_ptr->value, COLOR_RESET);
  } else if (val_ptr->type_tag == 4) {
    printf("%s%s%s", COLOR_STRING, (char*)val_ptr->value, COLOR_RESET);
  } else {
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
