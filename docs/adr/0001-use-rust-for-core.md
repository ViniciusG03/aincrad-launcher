# ADR-0001: Use Rust for the Launcher Core

## Status

Accepted

## Date

2026-08-19

## Context

Aincrad Launcher needs a core responsible for the main launcher operations, including:

- Communicating with Minecraft and Microsoft-related services
- Downloading and validating files
- Managing Minecraft installations and instances
- Interacting with the filesystem
- Managing Java runtimes
- Building Minecraft launch configurations
- Starting and monitoring external processes
- Handling concurrent operations
- Providing functionality to the future desktop interface

The core should remain reasonably independent from the graphical interface so that its functionality can initially be developed and tested through a command-line application.

The long-term desktop application is currently planned to use Tauri with a React and TypeScript frontend.

Because the developer has no previous Rust experience, this decision is also expected to introduce a significant learning challenge.

The decision should therefore be considered an informed initial architectural choice rather than an irreversible commitment.

---

## Decision Drivers

The main factors considered for this decision are:

- Integration with the planned Tauri desktop application
- Native access to operating system functionality
- Performance
- Memory safety
- Concurrency support
- Cross-platform development
- Ability to build the launcher core independently from the UI
- Maintainability of a long-running project
- Educational value
- Opportunity to study lower-level software engineering concepts

---

## Options Considered

### Rust

Rust is a systems programming language focused on performance, type safety, memory safety, and concurrency.

It integrates naturally with Tauri, which uses Rust for its native application layer.

Rust is well suited to the responsibilities expected from the launcher core, such as:

- Filesystem operations
- Network requests
- File downloads
- Hash validation
- Process management
- Concurrency
- Cross-platform system interaction

Rust also provides an opportunity to study concepts that are usually less visible in higher-level languages, particularly ownership, borrowing, memory management, explicit error handling, and concurrency.

#### Advantages

- Native integration with Tauri
- Strong compile-time guarantees
- Memory safety without requiring a garbage collector
- Good performance characteristics
- Strong tooling through Cargo
- Suitable for filesystem and process-heavy applications
- Strong type system
- Good support for concurrent applications
- Can produce native binaries
- Encourages explicit handling of errors
- Provides significant educational value

#### Disadvantages

- Significant learning curve
- Ownership and borrowing introduce unfamiliar concepts
- Development may initially be slower
- Compiler errors may be difficult to understand while learning
- Some implementation tasks may require more explicit code than in higher-level languages

---

### TypeScript

TypeScript was considered because the graphical interface is planned to use React and TypeScript.

Using TypeScript for both the frontend and the launcher logic could reduce the number of languages used in the project.

It also provides a productive development environment and a large ecosystem.

#### Advantages

- Easier initial development
- Same language as the planned frontend
- Large ecosystem
- Strong support for asynchronous programming
- Familiar web development tooling
- Lower initial learning cost

#### Disadvantages

- Would reduce the separation between the native launcher core and the frontend stack
- Native operating system operations would depend more heavily on runtime APIs, libraries, or an additional native integration layer
- Using Tauri would still introduce Rust into the application
- Provides less opportunity to study systems-level programming concepts
- Some guarantees provided by Rust would instead need to be enforced at runtime or through application design

TypeScript remains the planned language for the frontend but was not selected as the primary language for the launcher core.

---

### Java

Java was considered because Minecraft: Java Edition itself runs on the JVM and because Java is professionally relevant to the developer.

Java provides a mature ecosystem, strong tooling, cross-platform support, concurrency APIs, HTTP clients, filesystem APIs, and process management.

A complete launcher could reasonably be implemented using Java.

#### Advantages

- Mature ecosystem
- Excellent development tooling
- Strong cross-platform support
- Familiar object-oriented programming model
- Strong concurrency support
- Good networking and filesystem APIs
- Professionally relevant to the developer
- Natural relationship with the JVM used by Minecraft

#### Disadvantages

- Does not integrate as naturally with the currently planned Tauri architecture
- Would likely require either a separate Java process or a different desktop architecture
- Would add a JVM requirement for the launcher itself if distributed traditionally
- Choosing Java mainly because of career goals would prioritize language preference over the needs of the product
- Would provide less exposure to systems programming concepts that the project can be used to study

Java may still appear in tools, experiments, services, or future components when technically justified.

However, it was not selected for the launcher core simply because the developer intends to work professionally with Java.

---

## Decision

Rust will be used as the primary language for the Aincrad Launcher core.

The core will initially be developed as a command-line application without React or Tauri.

This allows the launcher functionality to be designed, understood, tested, and evolved independently from the graphical interface.

The expected evolution is:

```text
Rust CLI
    ↓
Launcher Core
    ↓
Stable and Tested Core
    ↓
Tauri Integration
    ↓
React + TypeScript UI
```

The graphical interface should consume capabilities provided by the core rather than contain the main launcher logic itself.

---

## Rationale

Rust was selected because it aligns well with both the technical requirements of the launcher and the currently planned desktop architecture.

The launcher core will perform many operations close to the operating system, including filesystem access, process management, networking, downloads, validation, and concurrency.

These responsibilities fit well with Rust.

The planned use of Tauri is another important factor because Rust is a fundamental part of Tauri's native application architecture.

The decision also supports the educational goals of Aincrad Launcher.

Learning Rust is expected to expose the developer to concepts such as:

- Ownership
- Borrowing
- Lifetimes
- Memory management
- Strong type modeling
- Explicit error handling
- Concurrency
- Systems programming

The objective is not to learn Rust simply for the sake of learning another language.

Rust was chosen because it appears to be a good technical fit for the product while also providing valuable software engineering challenges.

---

## Consequences

### Positive

The project will gain:

- A native launcher core
- Strong integration with Tauri
- Strong compile-time guarantees
- Explicit error handling
- Good control over system resources
- A clear separation between launcher logic and UI
- Opportunities to study systems programming
- Opportunities to study concurrency and resource management
- A core that can initially run independently as a CLI

### Negative

The project must accept:

- A slower initial development pace
- Additional learning requirements
- Increased cognitive complexity while learning Rust
- The need to understand ownership and borrowing
- The need to work with multiple languages once the frontend is introduced
- Potential development friction while the developer is still unfamiliar with the Rust ecosystem

These costs are considered acceptable because learning and technical understanding are primary goals of the project.

---

## Alternatives Reconsideration

This decision is not permanent.

The architecture should not continue using Rust simply because an ADR exists.

The decision should be reconsidered if experience shows that:

- Rust introduces complexity without providing sufficient benefits
- The Tauri architecture is replaced
- Another language provides a substantially better solution
- The core requirements change significantly
- Cross-platform limitations appear
- Maintenance becomes unnecessarily difficult

A future ADR may supersede this decision if necessary.

---

## Learning Review

Because Rust was selected before the developer had practical experience with the language, this ADR should be reviewed after the first meaningful launcher milestones.

At that point, the developer should be able to independently evaluate questions such as:

- Did Rust make filesystem operations easier or harder?
- How useful was the type system?
- Did ownership improve the design or mostly introduce friction?
- How was error handling affected?
- How well did Rust handle asynchronous operations?
- How difficult was testing?
- Does Rust still make sense for the launcher core?
- Would the same decision be made again?

The purpose of this review is to transform the initial technology recommendation into a decision the developer can personally understand and defend.
