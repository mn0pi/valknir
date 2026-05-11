# Valknir

Valknir is a static analysis engine for C programs, written in Rust.

It analyses source code for unsafe memory behaviour — buffer overflows, 
double frees, use-after-free, memory leaks — and produces output that 
explains not just what the vulnerability is, but why dangerous memory 
states emerge and how they can be exploited.

Built as a learning project at the intersection of systems programming, binary exploitation research, and compiler-style analysis. 
The process of building it is as much the point as the tool itself. 

---

## Vision

Most static analyzers are black boxes. They tell you something is wrong 
without showing you why, which is fine for production use but useless 
for actually understanding vulnerability classes.

Valknir is designed to be transparent by default:

- every detection is explainable
- every state transition is observable  
- the analysis logic is readable, not hidden behind framework abstractions

The goal is a tool that teaches as it analyses — useful for vulnerability 
research and for anyone trying to understand how memory corruption actually 
works at the level where exploits live.

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

## Author

mn0pi — software developer, learning systems programming and binary 
exploitation from first principles.

Valknir is documented as part of a broader research and writing practice 
at [7thmagpie.xyz](https://7thmagpie.xyz) — where the technical work 
connects to wider patterns in systems thinking.


