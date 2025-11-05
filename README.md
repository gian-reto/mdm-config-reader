# `@gian-reto/mdm-config-reader`

![https://github.com/napi-rs/package-template/actions](https://github.com/napi-rs/package-template/workflows/CI/badge.svg)

> A Node.js library for reading Mobile Device Management (MDM) configuration settings, built with Rust and napi-rs.

## Features

- 🔐 **Windows MDM Support**: Read configuration settings written by Microsoft Intune
- 🚀 **Native Performance**: Built with Rust for optimal performance
- 🌍 **Cross-Platform**: Works on Windows, macOS, and Linux (returns empty object on non-Windows platforms)
- 📦 **Zero Dependencies**: No additional runtime dependencies required
- 🔧 **TypeScript Support**: Full TypeScript definitions included

## Installation

```bash
npm install @gian-reto/mdm-config-reader
# or
pnpm add @gian-reto/mdm-config-reader
# or
yarn add @gian-reto/mdm-config-reader
```

## Usage

### JavaScript

```javascript
const { getMdmSettings } = require('@gian-reto/mdm-config-reader')

const settings = getMdmSettings()
console.log('MDM Settings:', settings)
// Example output on Windows with Intune config:
// { "AppTimeout": "30", "LogLevel": "debug", "EnableFeatureX": "true" }

// On macOS/Linux or Windows without MDM config:
// {}
```

### TypeScript

```typescript
import { getMdmSettings } from '@gian-reto/mdm-config-reader'

const settings: Record<string, string> = getMdmSettings()

// All values are strings, regardless of their original type in the MDM store
const timeout = parseInt(settings['AppTimeout'] || '60')
const enableFeature = settings['EnableFeatureX'] === 'true'
```

## API

### `getMdmSettings(): Record<string, string>`

Reads MDM configuration settings for the current application.

**Platform Behavior:**

- **Windows**: Retrieves settings from the `Managed.App.Settings` container written by Microsoft Intune
- **macOS/Linux**: Returns an empty object `{}`

**Returns:**

- A JavaScript object containing key-value pairs where all values are strings
- Returns an empty object `{}` if:
  - No MDM configuration is found
  - The application is not running in a package context (Windows)
  - On non-Windows platforms (macOS/Linux)
  - Any Windows API calls fail

**Error Handling:**

This function never throws errors. It gracefully handles all failure cases by returning an empty object. This makes it safe to call without try-catch blocks.

**Note:** All configuration values are converted to strings, regardless of their original type (int, bool, string) in the MDM store. This provides a consistent interface and allows you to parse/validate values in your application logic.

## Platform Support

This library currently supports:

- ✅ Windows (x64, x86, ARM64) - Full MDM reading support
- ✅ macOS (x64, ARM64) - Returns empty object (future support planned)
- ✅ Linux (x64, ARM64, ARM) - Returns empty object (future support planned)

## Development

### Prerequisites

- Rust (latest stable)
- Node.js 16+
- pnpm (run `corepack enable`)

On NixOS, you can use the provided Nix flake:

```bash
nix develop
```

### Building

```bash
# Development build
pnpm build:debug

# Production build
pnpm build
```

### Testing

```bash
# Run tests
pnpm test

# Run simple test
node simple-test.js
```

### Project Structure

```
├── src/
│   ├── lib.rs       # Main entry point with platform dispatch
│   └── windows.rs   # Windows-specific implementation
├── __test__/        # Test files
├── index.d.ts       # Auto-generated TypeScript definitions
└── index.js         # Auto-generated JavaScript loader
```

## How It Works

This library uses [napi-rs](https://napi.rs/) to create a native Node.js addon written in Rust. On Windows, it uses the [windows-rs](https://github.com/microsoft/windows-rs) crate to access the Windows Runtime API and read from the application's `LocalSettings` container, specifically the `Managed.App.Settings` container where Microsoft Intune writes MDM configuration.

The library supports various property types from the MDM store:

- Integers (Int16, Int32, Int64, UInt16, UInt32, UInt64)
- Booleans
- Strings

All values are converted to strings for a consistent JavaScript API.

## License

MIT

## Contributing

Contributions are welcome! This is currently a proof-of-concept with Windows support. Future plans include:

- macOS support (Jamf, other MDM solutions)
- Linux support (various MDM solutions)

Please open an issue to discuss any changes before submitting a PR.
