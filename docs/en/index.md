---
# https://vitepress.dev/reference/default-theme-home-page
layout: home
lang: en-US

head:
  - - meta
    - name: description
      content: "LightVM is a capability-based virtual machine designed for safe, predictive, and efficient bytecode execution with a minimal memory footprint."
  - - meta
    - name: keywords
      content: "LightVM, bytecode execution, virtual machine, security, systems programming, capability-based"
  - - meta
    - property: og:title
      content: "LightVM | Secure & Optimized Bytecode VM"
  - - meta
    - property: og:description
      content: "Minimalist execution for maximal security. Explore LightVM documentation."
  - - meta
    - property: og:url
      content: "https://lightvm.vercel.app/"

title: Main Documentation
description: A capability-based virtual machine designed for secure, predictable, and optimized bytecode execution.

hero:
  name: "LightVM"
  text: "A capability-based virtual machine designed for secure, predictable, and optimized bytecode execution."
  tagline: Minimalist Execution. Maximal Security.
  actions:
    - theme: brand
      text: Get Started
      link: /get-started/installation
    - theme: alt
      text: API References
      link: /api-references/method-functions/run-method

features:
  - title: Zero Magic (Deterministic)
    details: Instruction execution is linear and completely predictable. The VM operates explicitly, executing instructions exactly as they are defined.
  - title: Resource Conscious
    details: Designed with a minimal memory footprint through the use of optimized data structures such as SmolStr and ahash for fast metadata management.
  - title: Explicit Security
    details: Security is managed through a strict Capability system. Every VM access and operation must have permissions explicitly defined by the host from the outset.
---

<HeroStats />