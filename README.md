# Valknir

> Experimental static analysis engine for low-level memory vulnerability research.

Valknir is a systems-focused static analyzer written in Rust that explores unsafe memory behaviour in C programs through AST-driven analysis.

The long-term goal is not just to detect vulnerabilities, but to build a transparent analysis engine that exposes *why* dangerous memory states emerge.

---

# Vision

Modern static analyzers are often opaque, heavyweight, or inaccessible to learners.

Valknir is designed differently:

- minimal core architecture
- transparent internal state tracking
- AST-first analysis
- educational by design
- focused on real memory corruption primitives

The project doubles as:

1. a practical vulnerability research tool
2. a public systems programming study
3. a deep dive into compiler-style analysis pipelines

---

# Current Capabilities

## Parsing

- Parses C source code using Tree-sitter
- Walks the full AST recursively
- Detects function calls
- Extracts identifiers and arguments

## Memory Tracking

Currently implemented:

- malloc() detection
- free() detection
- allocation state tracking
- double free detection

Example:

```c
int *p = malloc(64);
free(p);
free(p);
```

Produces:

```text
[valknir] double free detected:
Pointer: p
Allocated at: 2
First free at: 3
Second free at: 4
```

---

# Planned Analysis Features

## Memory Safety

- use-after-free detection
- memory leak detection
- invalid free detection
- null dereference analysis
- stack lifetime misuse
- heap ownership tracking

## Control/Data Flow

- CFG generation
- lightweight SSA-inspired IR
- taint propagation
- symbolic state tracking
- interprocedural analysis

## Exploit-Oriented Research

- allocator abuse patterns
- heap corruption heuristics
- stack corruption indicators
- dangerous libc call detection
- exploit primitive classification

---

# Architecture

```text
Source Code
    ↓
Tree-sitter Parser
    ↓
AST Traversal
    ↓
IR / State Tracking
    ↓
Analysis Passes
    ↓
Diagnostics
```

---

# Project Philosophy

Valknir prioritises:

- clarity over abstraction
- understanding over automation
- systems-level thinking
- incremental complexity

The project intentionally avoids hiding analysis logic behind large frameworks.

Every detection should be explainable.

Every state transition should be observable.

---

# Tech Stack

## Language

- Rust

## Parsing

- tree-sitter
- tree-sitter-c

## CLI

- clap

---

# Development Roadmap

## Phase 1 — AST Foundations

- [x] Parse C files
- [x] Traverse AST
- [x] Detect malloc/free
- [x] Track allocation state
- [x] Detect double free

## Phase 2 — Stateful Analysis

- [ ] Use-after-free detection
- [ ] Leak detection
- [ ] Ownership modelling
- [ ] Stack object tracking

## Phase 3 — Intermediate Representation

- [ ] Design custom IR
- [ ] Lower AST into IR
- [ ] Add control-flow awareness
- [ ] Track symbolic states

## Phase 4 — Advanced Research

- [ ] Taint analysis
- [ ] Constraint reasoning
- [ ] Interprocedural analysis
- [ ] Exploit primitive modelling

---

# Why Rust?

Rust forces explicit reasoning about:

- ownership
- mutation
- aliasing
- lifetime boundaries
- state transitions

Those are the same concepts required for reliable vulnerability analysis.

Building Valknir in Rust is therefore part of the research process itself.

---

# Example Usage

```bash
cargo run -- analyse examples/double_free.c
```

---

# Current Status

Valknir is in early development.

The codebase is intentionally evolving in public as part of a long-term systems and vulnerability research journey.

Expect rapid architectural changes.

---

# Contributing

This project is currently highly experimental.

Contributions, ideas, critiques, and vulnerability research discussions are welcome.

---

# Author

Created by mn0pi.

Systems programming, static analysis, memory safety research.



