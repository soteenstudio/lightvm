---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "LightVM Documentation"
  text: "A capability-based virtual machine designed for secure, predictable, and optimized bytecode execution."
  tagline: My great project tagline
  actions:
    - theme: brand
      text: Get Started
      link: /getStarted/installation
    - theme: alt
      text: API References
      link: /api-references

features:
  - title: Zero Magic (Deterministic)
    details: Instruction execution is linear and completely predictable. The VM operates explicitly, executing instructions exactly as they are defined.
  - title: Resource Conscious
    details: Designed with a minimal memory footprint through the use of optimized data structures such as SmolStr and ahash for fast metadata management.
  - title: Explicit Security
    details: Security is managed through a strict Capability system. Every VM access and operation must have permissions explicitly defined by the host from the outset.
---

