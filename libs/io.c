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

void lightvm_print(long long val) { 
  printf("%lld", val); 
}

void lightvm_println(long long val) { 
  printf("%lld\n", val); 
}

void lightvm_stdout(long long val) { 
  printf("%lld", val); 
}

void lightvm_stdoutln(long long val) { 
  printf("%lld\n", val); 
}

void* lightvm_stdin(void* buf) { 
  if (buf) {
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
