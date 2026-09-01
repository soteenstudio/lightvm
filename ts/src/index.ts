*** Begin Patch
*** Update File: ts/src/index.ts
@@
-  embedded(): VMResult {
-    return this.wrap(() => {
-      this.instance.clear_outputs();
-      this.instance.run({});
-      return {
-        value: undefined,
-        outputs: this.instance.get_outputs(),
-        halted: true,
-      };
-    });
-  }
+  /**
+   * Run the VM in an "embedded" one-shot mode: optionally clear previous outputs,
+   * execute the loaded program, collect outputs, and return a standardized result.
+   *
+   * Options:
+   *  - preserveOutputs: if true, does not clear existing outputs before running.
+   *  - exportName: if provided (string or true), attempt to call an export by name
+   *    after execution and include its return value in the `value` field. If true,
+   *    the export name `"main"` is used.
+   */
+  embedded(options?: { preserveOutputs?: boolean; exportName?: string | true }): VMResult {
+    return this.wrap(() => {
+      const preserve = options?.preserveOutputs ?? false;
+
+      if (!preserve) {
+        this.instance.clear_outputs();
+      }
+
+      this.instance.run({});
+
+      const outputs: string[] = this.instance.get_outputs();
+      let value: any = undefined;
+
+      if (options?.exportName) {
+        const name = options.exportName === true ? 'main' : options.exportName;
+        try {
+          const raw = this.instance.callExport(name, []);
+          if (raw == null || raw === 'Undefined') {
+            value = undefined;
+          } else {
+            value = raw;
+          }
+        } catch (e) {
+          // Do not throw from embedded for missing exports; keep value undefined
+          // and allow callers to inspect outputs. If callers want strict behavior,
+          // they should call export(name).call(...) directly.
+          value = undefined;
+        }
+      }
+
+      return {
+        value: value,
+        outputs,
+        halted: true,
+      };
+    });
+  }
*** End Patch
