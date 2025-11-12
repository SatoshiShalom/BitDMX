# Docker Setup

Complete Docker configuration for BitVMX-Z development and deployment.

## Architecture

```
┌─────────────────────────────────────────┐
│          Nginx (Frontend)               │
│     React + Vite Dashboard              │
│          Port: 5173 → 80                │
└─────────────┬───────────────────────────┘
              │ /api proxy
┌─────────────┴───────────────────────────┐
│      Backend (Rust/Axum)                │
│   Rollup Node + API + Aggregator        │
│           Port: 3000                    │
└─────────────┬───────────────────────────┘
              │
┌─────────────┴───────────────────────────┐
│       SQLite Database                   │
│     Persistent Volume: ./data           │
└─────────────────────────────────────────┘
```

## Files

- `Dockerfile.backend` - Multi-stage build for Rust backend
- `Dockerfile.frontend` - Multi-stage build for React frontend with Nginx
- `docker-compose.yml` - Orchestration configuration
- `frontend/nginx.conf` - Nginx reverse proxy config

## Usage

### Start Services

```bash
./scripts/start.sh
# or
docker-compose up -d
```

### View Logs

```bash
docker-compose logs -f

# Specific service
docker-compose logs -f backend
docker-compose logs -f frontend
```

### Stop Services

```bash
./scripts/stop.sh
# or
docker-compose down
```

### Rebuild Images

```bash
docker-compose up --build
```

### Access Services

- **Frontend**: http://localhost:5173
- **Backend API**: http://localhost:3000
- **Health Check**: http://localhost:3000/health

## Volumes

- `./data` - Persistent database storage

## Environment Variables

### Backend

- `RUST_LOG` - Log level (default: debug)
- `DATABASE_URL` - SQLite database path

### Frontend

- `VITE_API_URL` - Backend API URL (default: http://backend:3000)

## Production Deployment

For production, update `docker-compose.yml`:

```yaml
services:
  backend:
    environment:
      - RUST_LOG=info
    restart: unless-stopped

  frontend:
    restart: unless-stopped
```

Then deploy:

```bash
docker-compose -f docker-compose.yml up -d
```

## Troubleshooting

### Port Already in Use

Change ports in `docker-compose.yml`:

```yaml
ports:
  - "8080:3000"  # Backend
  - "8000:80"    # Frontend
```

### Database Issues

Reset database:

```bash
docker-compose down -v
rm -rf ./data
docker-compose up -d
```

### Build Failures

Clean and rebuild:

```bash
docker-compose down
docker system prune -a
docker-compose up --build
```
