# Roadmap

This roadmap represents the current development direction of Aincrad Launcher.

It is intentionally flexible and may change as technical discoveries, architectural decisions, and new requirements emerge.

The project prioritizes learning and understanding over delivery speed.

Milestone 0 — Project Bootstrap

- Project vision
- AI development guidelines
- Public README
- Roadmap
- Initial ADRs
- Rust workspace/bootstrap

Milestone 1 — Minecraft Version Discovery

- Fetch official version manifest
- Parse JSON
- Model version data
- Display versions in CLI
- Handle HTTP and parsing errors
- Add tests

Milestone 2 — Version Metadata

- Select a Minecraft version
- Fetch version-specific metadata
- Understand libraries, assets and client metadata
- Validate downloaded metadata

Milestone 3 — Minecraft Installation

- Create installation directories
- Download client files
- Download libraries
- Download assets
- Validate hashes
- Add progress reporting

Milestone 4 — Java Runtime

- Detect installed Java
- Understand Minecraft Java requirements
- Validate Java versions
- Eventually manage Java runtimes automatically

Milestone 5 — Launch Minecraft

- Build JVM arguments
- Build classpath
- Handle natives
- Start Minecraft process
- Capture logs and exit codes

Milestone 6 — Authentication

- Microsoft authentication
- Xbox/XSTS flow
- Minecraft Services
- Secure token storage

Milestone 7 — Instances

- Separate Minecraft installations
- Instance configuration
- Memory/JVM settings
- Game directories

Milestone 8 — Reliable Core

- Better error model
- Logging
- Retry strategy
- Download concurrency
- Caching
- More tests

Milestone 9 — Desktop Foundation

- Introduce Tauri
- Connect Rust core to desktop layer

Milestone 10 — User Interface

- React + TypeScript
- Launcher home
- Instance selection
- Settings
- Download progress

Milestone 11 — Mods and Modpacks

- Loader support
- Mods
- Modpacks
- Dependency handling

Milestone 12 — Cross-platform & Distribution

- Windows
- Linux
- Packaging
- Auto-update
- Release pipeline
