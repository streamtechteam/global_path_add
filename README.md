# Global Path Add

A Rust utility for easily adding directories to your system's PATH environment variable across different shell environments.

## Project Status

**⚠️ PRE-ALPHA - WORK IN PROGRESS ⚠️**

This project is in a pre-alpha stage and has significant limitations:

- Only works with absolute paths (relative paths not supported)
- Fish shell support is not implemented (files are created but not functional)
- Not thoroughly tested
- May have bugs or unexpected behavior

Use at your own risk and be sure to backup your shell configuration files.

## Overview

Global Path Add is a command-line tool that simplifies the process of permanently adding directories to your PATH environment variable. Unlike temporary solutions that only work for the current session, this tool makes persistent changes that apply to all new terminal sessions.

The tool is planned to support multiple shell environments:
- Bash (partially implemented)
- Fish shell (not yet implemented)
- Other shells (planned through a common configuration approach)

## How It Works (Current Implementation)

When you run `global_path_add` with an **absolute directory path**, it:

1. Creates a configuration directory at `~/.gpath_add/vars/`
2. Generates shell-specific configuration files:
   - `bash.sh` for Bash shell configurations
   - `fish.sh` for Fish shell configurations (currently just a placeholder)
   - `vars.sh` for a universal source file
3. Modifies your `~/.bashrc` file to source the configuration files
4. Adds your specified **absolute** directory path to the PATH environment variable

**Note:** Fish shell support is not yet implemented. The tool currently focuses on Bash shell integration.

## Installation

### Prerequisites

- Rust and Cargo installed on your system

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd global_path_add

# Build the project
cargo build --release

# The executable will be located at:
# target/release/global_path_add
```

### Installing

To install the tool globally on your system:

```bash
cargo install --path .
```

## Usage

```bash
global_path_add /absolute/path/to/directory
```

**Important:** This tool currently only works with absolute paths.

For example, to add a specific directory:

```bash
global_path_add /usr/local/myapp/bin
```

**Note:** The following example will NOT work because it's a relative path:
```bash
# This will not work correctly in the current implementation
global_path_add .
```

After running the command, restart your terminal or source your shell configuration:

```bash
# For bash
source ~/.bashrc

# Or source the generated files directly
source ~/.gpath_add/vars/vars.sh
```

## Configuration Files

The tool creates the following files in your home directory:

- `~/.gpath_add/vars/bash.sh` - Bash-specific PATH modifications (implemented)
- `~/.gpath_add/vars/fish.sh` - Fish shell PATH modifications (placeholder only - not implemented)
- `~/.gpath_add/vars/vars.sh` - Universal source file that loads both bash and fish configurations

It also modifies your `~/.bashrc` file to automatically source the configuration on shell startup.

**Note:** Fish shell support is not yet implemented. The `fish.sh` file is created but not populated with functional code.

## How to Verify

After adding a directory to your PATH and sourcing the configuration, you can verify it was added correctly:

```bash
echo $PATH
```

The directory you added should appear in the PATH output.

## Contributing

1. Fork the repository
2. Create a new branch for your feature
3. Make your changes
4. Run tests to ensure nothing is broken
5. Submit a pull request

**Note:** As this is a pre-alpha project, significant changes may occur that could affect your contributions.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Troubleshooting

### Command not found after installation

Make sure Cargo's bin directory is in your PATH. Add this line to your shell configuration:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Changes not taking effect

After running the tool, make sure to either:
1. Start a new terminal session, or
2. Source your shell configuration: `source ~/.bashrc`

### Warning: Pre-Alpha Status

This project is in pre-alpha and may have bugs or unexpected behavior. Before using it:
1. Backup your shell configuration files (`~/.bashrc`, etc.)
2. Check the generated files in `~/.gpath_add/vars/` to ensure they contain what you expect
3. Test in a non-production environment first