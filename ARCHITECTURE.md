# Architecture

## Overview

Aincrad Launcher is currently being developed as a Rust command-line application.

The initial goal is to build and understand the launcher core independently from any graphical interface.

The architecture is expected to evolve incrementally as real requirements appear.

This document describes both:

- The current architecture
- The planned long-term direction

Planned components should not be considered implemented until they actually exist in the codebase.

---

# Current Architecture

At the current stage, Aincrad Launcher is a minimal Rust binary application.

```text
User
  │
  ▼
Rust CLI
  │
  ▼
main()
```

Current project structure:

```text
aincrad-launcher/
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── main.rs
│
├── PROJECT.md
├── AGENTS.md
├── README.md
├── ROADMAP.md
├── ARCHITECTURE.md
│
└── docs/
    └── adr/
        └── 0001-use-rust-for-core.md
```

At this point, no Minecraft-specific functionality has been implemented yet.

The application currently serves only as the initial bootstrap for the Rust core.

---

# Architectural Direction

The launcher will initially evolve around a Rust core responsible for Minecraft-related functionality.

The command-line interface will act as the first interface to this core.

```text
┌──────────────────────────────┐
│          Rust CLI            │
│                              │
│ Development interface used   │
│ to interact with the core    │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│        Launcher Core         │
│                              │
│ Minecraft functionality      │
│ and system integration       │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│       External Systems       │
│                              │
│ Minecraft services           │
│ Microsoft services           │
│ Filesystem                   │
│ Network                      │
│ Java / JVM                   │
│ Operating System             │
└──────────────────────────────┘
```

The CLI is not intended to become the final user interface.

Its purpose is to allow launcher functionality to be developed, tested, and understood without depending on a graphical application.

---

# Planned Core Responsibilities

The Rust core is expected to eventually handle responsibilities such as:

## Minecraft Version Management

- Discover available Minecraft versions
- Retrieve version metadata
- Resolve version-specific configuration

## Download Management

- Download Minecraft client files
- Download libraries
- Download assets
- Validate file integrity
- Handle retries and failures
- Report progress

## Instance Management

- Create Minecraft instances
- Maintain isolated game directories
- Store instance configuration
- Manage version-specific settings

## Java Runtime Management

- Detect available Java installations
- Determine runtime compatibility
- Select the correct runtime
- Eventually install runtimes when necessary

## Authentication

- Authenticate legitimate Microsoft accounts
- Communicate with Microsoft and Minecraft authentication services
- Manage authentication state securely

## Process Management

- Build Minecraft launch commands
- Configure JVM arguments
- Build the classpath
- Start Java processes
- Capture process output
- Handle exit status and failures

## Filesystem Management

- Create required directories
- Read and write configuration
- Manage launcher data
- Validate existing files
- Handle platform-specific paths when necessary

These responsibilities are currently planned and should only become architectural components when implementation requirements justify them.

---

# External Systems

Aincrad Launcher will interact with several systems outside the application.

```text
                    Aincrad Launcher Core
                           │
          ┌────────────────┼────────────────┐
          │                │                │
          ▼                ▼                ▼
     Minecraft          Microsoft       Filesystem
      Services           Services
          │
          ▼
        Java / JVM
          │
          ▼
 Minecraft: Java Edition
```

External integrations should be isolated from unrelated application logic whenever reasonably possible.

This should make external behavior easier to understand, test, and replace when necessary.

---

# Planned Desktop Architecture

The long-term desktop application is currently planned to use:

- Rust for launcher core functionality
- Tauri for the native desktop application layer
- React for the graphical interface
- TypeScript for frontend development

The expected high-level architecture is:

```text
┌───────────────────────────────┐
│      React + TypeScript       │
│                               │
│       User Interface          │
└───────────────┬───────────────┘
                │
                │ Tauri IPC
                ▼
┌───────────────────────────────┐
│             Tauri             │
│                               │
│ Native desktop integration    │
└───────────────┬───────────────┘
                │
                ▼
┌───────────────────────────────┐
│          Rust Core            │
│                               │
│ Launcher functionality        │
└───────────────┬───────────────┘
                │
                ▼
┌───────────────────────────────┐
│       External Systems        │
│                               │
│ Minecraft APIs                │
│ Microsoft APIs                │
│ Filesystem                    │
│ Java / JVM                    │
│ Operating System              │
└───────────────────────────────┘
```

The graphical interface should consume functionality provided by the launcher core.

Minecraft-specific business logic should not unnecessarily live inside React components or UI-specific code.

---

# Dependency Direction

The project should aim for a dependency direction where user interfaces depend on launcher functionality rather than launcher functionality depending on user interfaces.

Conceptually:

```text
CLI ──────┐
          │
          ▼
     Launcher Core
          ▲
          │
Desktop ──┘
```

This should allow different interfaces to interact with the same launcher functionality.

The exact implementation of this separation has not yet been decided.

No additional abstraction should be introduced until there is a concrete need for it.

---

# Initial Development Flow

The current development direction is:

```text
Rust bootstrap
      ↓
Minecraft Version Discovery
      ↓
Version Metadata
      ↓
Minecraft Installation
      ↓
Java Runtime Management
      ↓
Minecraft Launching
      ↓
Authentication
      ↓
Instance Management
      ↓
Core Reliability
      ↓
Tauri Integration
      ↓
React + TypeScript UI
```

The detailed project milestones are maintained in [`ROADMAP.md`](ROADMAP.md).

---

# Architectural Principles

## Core Before UI

Launcher functionality should initially be developed without depending on the graphical interface.

This allows the system behavior to be understood and tested independently.

## Start Simple

The project should begin with the simplest design capable of solving the current problem.

## Avoid Premature Abstraction

Modules, layers, interfaces, traits, patterns, and abstractions should not be introduced only because they may be useful someday.

They should solve concrete problems.

## Explicit Responsibilities

As the application grows, components should have understandable responsibilities.

A module should exist because it represents meaningful behavior or ownership of a concern.

## External Systems Should Be Visible

Network calls, filesystem access, process creation, and other side effects should be identifiable in the architecture.

## Errors Are Part of the Design

Failures should not be treated as exceptional afterthoughts.

Network requests, downloads, files, authentication, and external processes can fail and should be modeled accordingly.

## Cross-Platform Awareness

Initial development targets macOS.

However, operating-system assumptions should not unnecessarily leak throughout the core because Windows and Linux support are planned.

## Evidence-Driven Evolution

Architecture should evolve based on real implementation experience.

A theoretically elegant architecture is less valuable than one that solves actual problems clearly.

---

# Architecture Decision Records

Significant architectural decisions should be recorded in:

```text
docs/adr/
```

Current ADRs:

- [`ADR-0001: Use Rust for the Launcher Core`](docs/adr/0001-use-rust-for-core.md)

An ADR explains why a decision was made.

This document describes how the system currently fits together.

---

# Architecture Evolution

This document is expected to change frequently during the early stages of the project.

When implementation introduces meaningful architectural changes, this document should be reviewed.

Examples include:

- Creating new core modules
- Splitting the binary and library code
- Introducing multiple Rust crates
- Adding Tauri
- Adding persistence
- Adding authentication
- Introducing platform abstractions
- Changing dependency direction
- Replacing major technologies

The repository should reflect the architecture that actually exists rather than an idealized future design.

---

# Current Architecture Status

As of the initial bootstrap:

```text
Implemented:

Rust binary application
└── src/main.rs

Planned:

Rust launcher core
├── Version management
├── Downloads
├── Filesystem
├── Java runtime management
├── Process management
├── Authentication
└── Instances

Future:

Tauri
└── React + TypeScript UI
```

The next architectural milestone is the implementation of Minecraft version discovery.
