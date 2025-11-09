FROM golang:1.21-alpine AS builder

WORKDIR /app

# Copy go mod files
COPY go.mod go.sum* ./
RUN go mod download

# Copy source code
COPY *.go ./

# Build
RUN CGO_ENABLED=1 GOOS=linux go build -a -installsuffix cgo -o sync-server .

# Final stage
FROM alpine:latest

RUN apk --no-cache add ca-certificates sqlite

WORKDIR /root/

# Copy binary from builder
COPY --from=builder /app/sync-server .

# Create data directory
RUN mkdir -p /data

# Expose port
EXPOSE 3000

# Set default environment variables
ENV PORT=3000
ENV DB_PATH=/data/sync_server.db

CMD ["./sync-server"]
