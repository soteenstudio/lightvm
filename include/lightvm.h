/*
 * Copyright 2025-2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

#ifndef LIGHTVM_H
#define LIGHTVM_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct LightVM LightVM;
typedef struct LightVMExport LightVMExport;

/* Same values as Capability in the TS interface. */
typedef enum {
  LIGHTVM_CAPABILITY_CONTROL = 0,
  LIGHTVM_CAPABILITY_OBSERVE = 1,
  LIGHTVM_CAPABILITY_DEBUG = 2,
  LIGHTVM_CAPABILITY_UNSAFE = 3,
} LightVMCapability;

/* Same values as VMEvent in the TS interface. */
typedef enum {
  LIGHTVM_EVENT_TICK = 0,
  LIGHTVM_EVENT_HALT = 1,
  LIGHTVM_EVENT_PANIC = 2,
  LIGHTVM_EVENT_START = 3,
  LIGHTVM_EVENT_FINISH = 4,
} LightVMEvent;

/* Same values as TimeBudget in the TS interface. DEFAULT keeps the VM default. */
typedef enum {
  LIGHTVM_TIME_BUDGET_DEFAULT = -1,
  LIGHTVM_TIME_BUDGET_CHEAP = 0,
  LIGHTVM_TIME_BUDGET_NORMAL = 1,
  LIGHTVM_TIME_BUDGET_EXPENSIVE = 2,
} LightVMTimeBudget;

#define LIGHTVM_OK 0
#define LIGHTVM_ERR (-1)

typedef struct {
  bool nightly;
} LightVMRuntimeConfig;

typedef struct {
  bool backtrace;
  bool explain;
  bool hint;
  bool diagnostic_links;
} LightVMErrorOptions;

typedef struct {
  size_t max_io;
  size_t max_import;
  size_t max_alloc;
  size_t max_call;
  size_t max_jump;
  uint64_t max_ticks;
  size_t max_stack_size;
  const char *const *allowed_imports;
  size_t allowed_imports_len;
  bool unsafe_mode;
  int time_budget; /* LightVMTimeBudget */
} LightVMSecurityConfig;

typedef struct {
  const int *caps; /* array of LightVMCapability */
  size_t caps_len;
  LightVMRuntimeConfig runtime_config;
  LightVMErrorOptions error_options;
  LightVMSecurityConfig security_config;
} LightVMConfig;

typedef struct {
  bool capture_return;
} LightVMRunOptions;

/* Same values as TargetArch and FileType in the TS interface. */
typedef enum {
  LIGHTVM_TARGET_ARCH_AARCH64 = 0,
} LightVMTargetArch;

typedef enum {
  LIGHTVM_FILE_TYPE_ASSEMBLY = 0,
  LIGHTVM_FILE_TYPE_BINARY = 1,
} LightVMFileType;

typedef struct {
  int target_arch; /* LightVMTargetArch */
  int file_type;   /* LightVMFileType */
  const char *path;
} LightVMCompileConfig;

/* event is the event name, payload_json is valid only during the call. */
typedef void (*lightvm_event_cb)(const char *event, const char *payload_json, void *user_data);

/* Errors and memory. */
const char *lightvm_last_error(void);
void lightvm_string_free(char *text);

/* Lifecycle. A NULL config uses lightvm_config_default(). */
LightVMConfig lightvm_config_default(void);
LightVM *lightvm_new(const LightVMConfig *config);
void lightvm_free(LightVM *vm);

/*
 * Chainable calls return the same vm on success and NULL on error, and a NULL
 * vm stays NULL, so they can be nested like method chaining in TS:
 *   lightvm_run(lightvm_load(lightvm_set_max_ticks(vm, 1000), src), NULL);
 * lightvm_last_error() keeps the original error.
 */
LightVM *lightvm_set_max_io(LightVM *vm, size_t value);
LightVM *lightvm_set_max_import(LightVM *vm, size_t value);
LightVM *lightvm_set_max_alloc(LightVM *vm, size_t value);
LightVM *lightvm_set_max_call(LightVM *vm, size_t value);
LightVM *lightvm_set_max_jump(LightVM *vm, size_t value);
LightVM *lightvm_set_max_ticks(LightVM *vm, uint64_t value);
LightVM *lightvm_set_max_stack_size(LightVM *vm, size_t value);
LightVM *lightvm_set_allowed_imports(LightVM *vm, const char *json_array);
LightVM *lightvm_set_time_budget(LightVM *vm, int budget);
LightVM *lightvm_with_unsafe_mode(LightVM *vm, bool enabled);
LightVM *lightvm_with_nightly(LightVM *vm, bool enabled);
LightVM *lightvm_with_backtrace(LightVM *vm, bool enabled);
LightVM *lightvm_with_explain(LightVM *vm, bool enabled);
LightVM *lightvm_with_hint(LightVM *vm, bool enabled);
LightVM *lightvm_with_diagnostic_links(LightVM *vm, bool enabled);

LightVM *lightvm_load(LightVM *vm, const char *source);
LightVM *lightvm_provide(LightVM *vm, const char *name, const char *value_json);
LightVM *lightvm_provide_object(LightVM *vm, const char *json_object);
LightVM *lightvm_on(LightVM *vm, int event, lightvm_event_cb callback, void *user_data);

/* Returned strings are JSON text. Free them with lightvm_string_free. NULL on error. */
char *lightvm_run(LightVM *vm, const LightVMRunOptions *options);
char *lightvm_inspect(LightVM *vm);
char *lightvm_paniclog(LightVM *vm);
char *lightvm_embedded(LightVM *vm);
/* Not available when the library is built with the "wasm" feature. */
char *lightvm_info(LightVM *vm);

/* A NULL config uses lightvm_compile_config_default(). Returns the output path. */
LightVMCompileConfig lightvm_compile_config_default(void);
char *lightvm_compile(LightVM *vm, const LightVMCompileConfig *config);

void lightvm_halt(LightVM *vm);
int lightvm_clear_paniclog(LightVM *vm);

/* Exports. Free the handle with lightvm_export_free before freeing the vm. */
LightVMExport *lightvm_export(LightVM *vm, const char *name);
int lightvm_export_is_function(const LightVMExport *handle);
char *lightvm_export_call(LightVM *vm, const LightVMExport *handle, const char *args_json);
void lightvm_export_free(LightVMExport *handle);

/* tools() */
char *lightvm_tools_optimize_bytecode(LightVM *vm, const char *input);
char *lightvm_tools_stringify_ltc(const char *input);
char *lightvm_tools_parse_ltc(const char *code);
char *lightvm_tools_parse_ltc_array(const char *code);

#ifdef __cplusplus
}
#endif

#endif /* LIGHTVM_H */
