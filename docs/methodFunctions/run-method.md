# Run Method
After initializing the VM and configuring your desired capabilities, you can load your bytecode and start the execution process.

## Using TypeScript
For **TypeScript**, you can pass raw bytecode arrays directly to the loader. It is recommended to use `optimizeBytecode` beforehand to ensure the VM runs the most efficient version of your instructions.

```ts
const raw = [
  ["push", 5],
  ["val", "x"],
  ["set", "x"]
];
vm.load(tools.optimizeBytecode(raw))
  .run();
```

## Using Rust
In **Rust**, you typically work with serialized bytecode strings (or buffers). Similar to the TypeScript implementation, always optimize your bytecode before loading it into the VM to maintain optimal performance.

```rust
let raw = r#"[
  ["push", 5],
  ["val", "x"],
  ["set", "x"]
]"#;
vm.load(tools.optimize_bytecode(raw).clone())
  .run(None);
```

::: tip
The ``.run()`` method is the final step in the execution pipeline. Ensure all necessary `capabilities` have been granted during initialization to avoid runtime security exceptions.
:::
