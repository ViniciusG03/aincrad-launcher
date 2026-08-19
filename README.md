# Aincrad Launcher

A modern, open-source Minecraft: Java Edition launcher focused on performance, modularity, customization, and user experience.

> **Status:** Early development — the project is currently being designed and bootstrapped.

## About

Aincrad Launcher is a personal interpretation of what a modern Minecraft launcher can be.

The project aims to provide a customizable and enjoyable experience for Minecraft: Java Edition players while supporting features such as multiple instances, modpacks, account management, automatic Java runtime management, and cross-platform compatibility.

Aincrad Launcher is also a long-term software engineering learning project. Its development is being used to explore real-world challenges involving architecture, networking, authentication, concurrency, file systems, process management, testing, security, and desktop application development.

## Goals

The long-term goals of Aincrad Launcher include:

- Modern and responsive user experience
- High performance
- Minecraft instance management
- Multiple account support
- Mod and modpack management
- Automatic Java runtime detection and installation
- Customizable launcher interface
- Cross-platform support
- Modular and maintainable architecture
- Secure Microsoft authentication
- Open-source development

## Planned Tech Stack

The current planned stack is:

- **Rust** — core launcher functionality
- **Tauri** — desktop application framework
- **React** — user interface
- **TypeScript** — frontend development
- **Java / JVM** — Minecraft: Java Edition runtime

The initial development will focus on building the launcher core as a Rust command-line application before introducing the graphical interface.

## Platform Support

The long-term goal is to support:

- macOS
- Windows
- Linux

Development will initially target **macOS**, which is currently the project's primary development environment.

Cross-platform compatibility should still be considered from the beginning.

## Project Philosophy

Aincrad Launcher follows a few core principles:

- Start simple and evolve gradually.
- Introduce complexity only when it solves a real problem.
- Understand technical decisions before abstracting them.
- Treat documentation as part of development.
- Prioritize maintainability and clarity.
- Learn through implementation, experimentation, debugging, and refactoring.
- Keep the project legitimate, transparent, and open source.

## Project Status

Aincrad Launcher is currently in its initial development stage.

The current focus is on:

- Defining the product vision
- Establishing development guidelines
- Designing the initial roadmap
- Documenting architectural decisions
- Bootstrapping the Rust core

No stable or usable release is available yet.

## Documentation

Project documentation will be maintained directly in the repository.

- [`PROJECT.md`](PROJECT.md) — product vision, goals, constraints, and principles
- [`AGENTS.md`](AGENTS.md) — AI-assisted development and learning guidelines
- `ROADMAP.md` — development milestones and project direction
- `ARCHITECTURE.md` — current system architecture _(coming soon)_
- `docs/adr/` — Architecture Decision Records _(coming soon)_

## Development

Development instructions will be added once the initial project structure and toolchain are established.

## Contributing

Aincrad Launcher is currently in an early learning and development phase.

Contribution guidelines will be defined as the project becomes more mature.

## Legal

Aincrad Launcher is an independent, unofficial project and is not affiliated with, endorsed by, or associated with Mojang Studios or Microsoft.

The project is not intended to bypass Minecraft ownership requirements or authentication mechanisms.

Minecraft and related trademarks belong to their respective owners.

## License

This project is open source.

See [`LICENSE`](LICENSE) for license information.
