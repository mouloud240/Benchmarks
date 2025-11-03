# syntax=docker/dockerfile:1

# Build stage
FROM golang:1.25-alpine AS build

WORKDIR /app

# Copy go.mod and go.sum files to download dependencies
COPY go.mod go.sum ./
RUN go mod download

# Copy the rest of the source code
COPY . .

# Build the application
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o /main ./cmd/api

# ---

# Final stage
FROM alpine:latest

# Copy the binary from the build stage
COPY --from=build /main /main

# Expose port 8080
EXPOSE 8080

# Run the binary
ENTRYPOINT ["/main"]
