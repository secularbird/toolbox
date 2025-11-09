# Sync Server

A lightweight Go server for syncing reminders with the Tauri app.

## Features

- ✅ RESTful API for CRUD operations
- ✅ SQLite database for persistent storage
- ✅ CORS enabled for cross-origin requests
- ✅ Health check endpoint
- ✅ Automatic database initialization
- ✅ Logging for all operations

## Quick Start

### Prerequisites

- Go 1.21 or higher
- SQLite3

### Installation

```bash
cd sync-server

# Download dependencies
go mod download

# Build the server
go build -o sync-server

# Run the server
./sync-server
```

Or run directly:

```bash
go run main.go
```

## Configuration

Configure via environment variables:

```bash
# Set custom port (default: 3000)
PORT=8080 go run main.go

# Set custom database path (default: ./sync_server.db)
DB_PATH=/path/to/database.db go run main.go

# Both
PORT=8080 DB_PATH=/data/reminders.db go run main.go
```

## API Endpoints

### Health Check
```bash
GET /health

Response:
{
  "status": "ok",
  "timestamp": 1699564800
}
```

### Get All Reminders
```bash
GET /api/reminders

Response: 200 OK
[
  {
    "id": 1,
    "title": "Meeting",
    "description": "Team meeting",
    "time": "2024-11-10T10:00:00",
    "completed": false,
    "category": "work",
    "frequency": "once",
    "created_at": "2024-11-09T10:00:00",
    "updated_at": "2024-11-09T10:00:00"
  }
]
```

### Get Single Reminder
```bash
GET /api/reminders/:id

Response: 200 OK
{
  "id": 1,
  "title": "Meeting",
  ...
}
```

### Create Reminder
```bash
POST /api/reminders
Content-Type: application/json

{
  "title": "Meeting",
  "description": "Team meeting",
  "time": "2024-11-10T10:00:00",
  "completed": false,
  "category": "work",
  "frequency": "once"
}

Response: 201 Created
{
  "id": 1,
  "title": "Meeting",
  ...
}
```

### Update Reminder
```bash
PUT /api/reminders/:id
Content-Type: application/json

{
  "title": "Updated Meeting",
  "description": "Updated description",
  "time": "2024-11-10T14:00:00",
  "completed": true,
  "category": "work",
  "frequency": "once"
}

Response: 200 OK
{
  "id": 1,
  "title": "Updated Meeting",
  ...
}
```

### Delete Reminder
```bash
DELETE /api/reminders/:id

Response: 200 OK
{
  "message": "Reminder deleted successfully"
}
```

## Testing

Test the server with curl:

```bash
# Health check
curl http://localhost:3000/health

# Get all reminders
curl http://localhost:3000/api/reminders

# Create a reminder
curl -X POST http://localhost:3000/api/reminders \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Test",
    "description": "Test reminder",
    "time": "2024-11-10T10:00:00",
    "completed": false,
    "category": "personal",
    "frequency": "once"
  }'

# Update a reminder
curl -X PUT http://localhost:3000/api/reminders/1 \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Test",
    "description": "Updated description",
    "time": "2024-11-10T10:00:00",
    "completed": true,
    "category": "personal",
    "frequency": "once"
  }'

# Delete a reminder
curl -X DELETE http://localhost:3000/api/reminders/1

# Get single reminder
curl http://localhost:3000/api/reminders/1
```

## Database Schema

The server automatically creates the following schema:

```sql
CREATE TABLE reminders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    time TEXT NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0,
    category TEXT NOT NULL,
    frequency TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_reminders_created_at ON reminders(created_at);
CREATE INDEX idx_reminders_category ON reminders(category);
```

## Deployment

### Using Docker

Create `Dockerfile`:

```dockerfile
FROM golang:1.21-alpine AS builder
WORKDIR /app
COPY go.* ./
RUN go mod download
COPY *.go ./
RUN go build -o sync-server

FROM alpine:latest
RUN apk --no-cache add ca-certificates sqlite
WORKDIR /root/
COPY --from=builder /app/sync-server .
EXPOSE 3000
CMD ["./sync-server"]
```

Build and run:

```bash
docker build -t sync-server .
docker run -p 3000:3000 -v $(pwd)/data:/data -e DB_PATH=/data/sync_server.db sync-server
```

### Using systemd (Linux)

Create `/etc/systemd/system/sync-server.service`:

```ini
[Unit]
Description=Reminder Sync Server
After=network.target

[Service]
Type=simple
User=nobody
WorkingDirectory=/opt/sync-server
ExecStart=/opt/sync-server/sync-server
Restart=on-failure
Environment="PORT=3000"
Environment="DB_PATH=/var/lib/sync-server/sync_server.db"

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable sync-server
sudo systemctl start sync-server
sudo systemctl status sync-server
```

## Production Considerations

1. **Use HTTPS**: Deploy behind a reverse proxy (nginx/caddy)
2. **Authentication**: Add JWT or API key authentication
3. **Rate Limiting**: Implement rate limiting to prevent abuse
4. **Database Backup**: Regular backups of SQLite database
5. **Monitoring**: Add logging and monitoring (Prometheus/Grafana)

## Development

Run with auto-reload using `air`:

```bash
go install github.com/cosmtrek/air@latest
air
```

## Troubleshooting

**Port already in use?**
```bash
# Find process using port 3000
lsof -i :3000

# Kill the process
kill -9 <PID>
```

**Database locked?**
- Ensure only one instance is running
- Check file permissions

**CORS issues?**
- Server has CORS enabled by default
- Check browser console for specific errors

## License

MIT
