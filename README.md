# Jsnap — Just Snap It

**Jsnap** is a lightweight, enterprise-safe screenshot and annotation tool.
No telemetry, no cloud sync, no Electron.

> “I'll jsnap it to you.”

## Features
- Instant capture (`PrintScreen`, `Alt+PrintScreen`)
- Crop, annotate, save — all vector-based
- Policy-locked destinations (ISRM compliant)
- Optional DPAPI or password-based encryption
- Append-only audit log
- < 10 MB static binary, < 20 MB RAM

## Highlights
- `PrintScreen` → crop → annotate → **paste directly into Word / Outlook / Teams**
- < 10 MB static binary, no installer dependencies
- Local-only, ISRM-safe design
- Optional encryption & policy for enterprise builds

## Quick Start
cargo build --release
target\release\jsnap.exe

Press **Print Screen**, crop, choose *Copy & Close*, then open Word and **Ctrl + V**.

## Enterprise Branding
Build as `AcmeSnap`:
SNAP_BRAND=AcmeSnap cargo build --release

All folders, registry keys, and policy paths update automatically.


## Admin Policy
System policy file (JSON):


jsnap/
├── Cargo.toml
├── build.rs
├── README.md
├── policy.schema.json
├── assets/
│   └── icons/jsnap.ico
└── src/
    ├── main.rs
    ├── brand.rs
    ├── util.rs
    ├── core/
    │   ├── mod.rs
    │   ├── capture.rs
    │   ├── crypto.rs
    │   └── policy.rs        ← next to implement
    ├── ui/
    │   ├── mod.rs
    │   └── hotkey.rs
    ├── export/
    │   ├── mod.rs
    │   ├── flatten.rs       ← writes the final image
    │   ├── clipboard.rs     ← multi-format clipboard copy
    │   └── save.rs
    └── history/
        ├── mod.rs
        ├── index.rs
        └── thumb.rs
