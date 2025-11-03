# Web Framework Benchmark

A comprehensive benchmarking suite to compare the performance of different web frameworks across various scenarios using [k6](https://k6.io/) load testing.

## Overview

This repository contains load tests and implementations for multiple web frameworks to determine which performs best under different conditions. Each framework implementation follows the same specification to ensure fair comparisons.

## Repository Structure

```
benchmark/
├── load-test/              # k6 load testing scripts
│   ├── plain-text.js       # Simple GET endpoint benchmark
│   └── parsing-validation.js # JSON POST endpoint benchmark
├── go/                     # Go implementation
├── fastapi/                # FastAPI (Python) implementation
├── django/                 # Django (Python) implementation
├── nest/                   # NestJS (Node.js/Fastify) implementation
└── rust/                   # Rust implementation 
```

## Test Scenarios

### 1. Plain Text Response (`load-test/plain-text.js`)

Tests a simple GET endpoint that returns plain data without complex processing.

- **Endpoint**: `GET /api/v1/greetings`
- **Load Profile**:
  - Ramp-up to 1,000 users over 20s
  - Ramp-up to 5,000 users over 40s
  - Ramp-up to 10,000 users over 40s
  - Maintain 10,000 users for 30s
  - Ramp-down to 0 users over 20s
- **Success Criteria**:
  - 95% of requests complete below 500ms
  - HTTP error rate < 1%

### 2. Parsing & Validation (`load-test/parsing-validation.js`)

Tests JSON parsing, validation, and response generation with a POST endpoint.

- **Endpoint**: `POST /api/v1/greetings`
- **Payload**:
  ```json
  {
    "id": 123456789,
    "name": "user-1-5",
    "message": "hello from vu 1 iter 5",
    "greetDate": "2025-11-03T10:00:00.000Z"
  }
  ```
- **Load Profile**: Same as plain text test
- **Success Criteria**: Same as plain text test

## Prerequisites

- [Docker](https://www.docker.com/) - For running framework implementations
- [k6](https://k6.io/) - For running load tests

### Installing k6

```bash
# macOS
brew install k6

# Linux
sudo gpg -k
sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D69
echo "deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main" | sudo tee /etc/apt/sources.list.d/k6.list
sudo apt-get update
sudo apt-get install k6

# Windows
choco install k6
```

## Running Tests

1. Start the server you want to test:

```bash
# Example: Testing the Go implementation
cd go
docker build -t benchmark-go .
docker run -p 3000:8080 benchmark-go
```

2. Run the desired load test:

```bash
# Plain text test
k6 run load-test/plain-text.js

# Parsing & validation test
k6 run load-test/parsing-validation.js

# Override the base URL if needed
BASE_URL=http://localhost:3000 k6 run load-test/plain-text.js
```

## Contributing

We welcome contributions of new framework implementations! To add a new framework:

### 1. Create Framework Directory

Create a new directory with your framework name:

```bash
mkdir my-framework
cd my-framework
```

### 2. Implement Required Endpoints

Your implementation must provide these two endpoints:

#### GET `/api/v1/greetings`

Returns a simple greeting response.

**Response Example**:
```json
{
  "message": "Hello, World!"
}
```

#### POST `/api/v1/greetings`

Accepts and validates a JSON payload.

**Request Body**:
```json
{
  "id": number,
  "name": string,
  "message": string,
  "greetDate": string (ISO 8601 date)
}
```

**Response** (2xx status code):
```json
{
  "success": true,
  "data": {
    "id": number,
    "name": string,
    "message": string,
    "greetDate": string
  }
}
```

### 3. Create Dockerfile

Create a `Dockerfile` that:
- Builds your application
- Exposes the appropriate port (map to 3000 when running)
- Runs the server

**Example Dockerfile structure**:
```dockerfile
FROM your-base-image

WORKDIR /app

# Copy dependencies files
COPY package.json ./

# Install dependencies
RUN install-command

# Copy source code
COPY . .

# Build (if necessary)
RUN build-command

# Expose port
EXPOSE 8080

# Run the server
CMD ["start-command"]
```

### 4. Test Your Implementation

Before submitting:

1. Build and run your Docker container:
   ```bash
   docker build -t benchmark-myframework .
   docker run -p 3000:8080 benchmark-myframework
   ```

2. Test the endpoints manually:
   ```bash
   # Test GET endpoint
   curl http://localhost:3000/api/v1/greetings

   # Test POST endpoint
   curl -X POST http://localhost:3000/api/v1/greetings \
     -H "Content-Type: application/json" \
     -d '{"id": 1, "name": "test", "message": "hello", "greetDate": "2025-11-03T10:00:00.000Z"}'
   ```

3. Run both k6 tests:
   ```bash
   k6 run load-test/plain-text.js
   k6 run load-test/parsing-validation.js
   ```

### 5. Submit Your Implementation

1. Fork the repository
2. Create a new branch: `git checkout -b add-myframework`
3. Add your implementation
4. Commit your changes: `git commit -am 'Add MyFramework implementation'`
5. Push to the branch: `git push origin add-myframework`
6. Submit a Pull Request with:
   - Framework name and version
   - Brief description of your implementation
   - Test results (k6 output summary)

## Benchmark Results

Results from running all tests will be documented here after sufficient implementations are added.

| Framework | Plain Text (avg) | Plain Text (p95) | Parsing & Validation (avg) | Parsing & Validation (p95) |
|-----------|------------------|------------------|----------------------------|---------------------------|
| Go        | TBD              | TBD              | TBD                        | TBD                       |
| FastAPI   | TBD              | TBD              | TBD                        | TBD                       |
| Django    | TBD              | TBD              | TBD                        | TBD                       |
| NestJS    | TBD              | TBD              | TBD                        | TBD                       |
| Rust      | Coming Soon      | Coming Soon      | Coming Soon                | Coming Soon               |

## Guidelines for Fair Comparison

- All implementations should use production-ready configurations
- No caching mechanisms unless they're framework defaults
- Logging should be minimal (errors only)
- Use the most recent stable version of each framework
- Run tests on the same hardware and conditions
- Close all unnecessary applications during testing

## Contact

For questions, suggestions, or contributions, feel free to reach out:

**Email**: mouloudhasrane@gmail.com

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
