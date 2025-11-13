# Scripts

Helper scripts for development and deployment.

## Available Scripts

### `start.sh`
Start all services with Docker Compose.

```bash
./scripts/start.sh
```

### `stop.sh`
Stop all running services.

```bash
./scripts/stop.sh
```

### `dev.sh`
Run backend and frontend in development mode with hot reload.

```bash
./scripts/dev.sh
```

### `build.sh`
Build all components locally (Rust + frontend).

```bash
./scripts/build.sh
```

### `test.sh`
Run all tests (Rust + frontend).

```bash
./scripts/test.sh
```

### `clean.sh`
Clean all build artifacts and dependencies.

```bash
./scripts/clean.sh
```

## Requirements

All scripts require:
- Bash shell
- Execute permissions (already set)

Individual scripts may require:
- Docker & Docker Compose (`start.sh`, `stop.sh`)
- Rust toolchain (`build.sh`, `test.sh`, `dev.sh`)
- Node.js (`build.sh`, `test.sh`, `dev.sh`)
