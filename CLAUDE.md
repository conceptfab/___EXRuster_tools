# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based EXR (OpenEXR) file processing toolkit with two main applications:

1. **readEXR** (`src/main.rs`) - Analyzes EXR files and extracts channel information, grouping channels by type (Beauty, Scene, Technical, Light, Cryptomatte, etc.)
2. **Thumbnail Generator** (`_tools/main_thump.rs`) - Converts EXR files to PNG thumbnails with color space processing and parallel conversion

## Build Commands

- **Build project**: `cargo build --release` or use `build.bat` (Polish script)
- **Check compilation**: `cargo check`
- **Run tests**: `cargo test` (standard Rust testing)
- **Run readEXR**: `cargo run --release` (uses `data/` folder by default)
- **Run thumbnail generator**: Compile then use `run_example.bat` or `release.py`

## Key Files and Architecture

### Core Application (`src/main.rs`)
- Processes EXR files in parallel using Rayon
- Configurable channel grouping via `channel_groups.json`
- Groups channels by: Base/Beauty, Scene, Technical, Light, Cryptomatte, Scene Objects
- Pattern matching for channel classification (prefixes and wildcards)
- Outputs detailed text analysis files

### Thumbnail Tool (`_tools/main_thump.rs`)
- CLI tool using Clap for argument parsing
- Parallel EXR to PNG thumbnail conversion
- Linear color space tone mapping with Reinhard algorithm
- Configurable gamma correction and scaling filters
- Detailed timing statistics and batch processing

### Configuration System
- `channel_groups.json` - Defines channel grouping rules with priorities
- Default config auto-generated if missing
- Supports custom prefixes, patterns, and group priorities

### Key Dependencies
- `exr = "1.73"` - EXR file reading/parsing
- `rayon = "1.8"` - Parallel processing
- `image = "0.24"` - Image manipulation and PNG output
- `clap = "4.4"` - CLI argument parsing
- `serde/serde_json` - Config serialization

## Development Patterns

### Error Handling
- Uses `std::result::Result<T, Box<dyn std::error::Error>>` for main functions
- String errors for processing functions: `Result<T, String>`
- Graceful fallbacks (e.g., default config on load failure)

### Performance Optimizations
- Parallel file processing with Rayon
- Arc-wrapped shared configuration for thread safety
- Atomic counters for statistics tracking
- Pre-allocated data structures where possible

### File Processing Workflow
1. Scan directory for EXR files
2. Load/create configuration
3. Process files in parallel
4. Generate output (text analysis or PNG thumbnails)
5. Collect and report statistics

## Data Folders
- `data/` - Input EXR files
- `tiff/` - Output TIFF files (legacy)
- `target/` - Rust build artifacts