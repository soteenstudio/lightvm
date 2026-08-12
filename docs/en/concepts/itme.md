# Itme (Benchmarking Tool)
**Itme** is a high-precision benchmarking utility designed for LightVM, providing developers with actionable insights into code performance. By leveraging statistical analysis and adaptive iteration cycles, **Itme** ensures reliable and reproducible measurements for your functions.

## How Itme Works
Itme utilizes an adaptive execution strategy to balance benchmarking duration with precision. It automatically calibrates the number of iterations required to reach a target execution window, followed by a rigorous statistical analysis of the samples to filter out noise and report accurate performance metrics.

 * **Adaptive Iteration**: Rather than using a fixed loop count, Itme dynamically adjusts the number of iterations until the measured time meets the `target_time`. This ensures both short-running and long-running functions receive a statistically significant sample size.
