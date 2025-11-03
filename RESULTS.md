# Framework Benchmark Results

## Introduction

This document contains benchmark results for various backend frameworks under identical load test conditions using **k6**.  
All tests return a **plain JSON response** to ensure consistency and fair comparison between frameworks.

---

## Plain JSON Response

### 1. NestJS (Fastify)
| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 485.8 ms |
|               | `http_req_failed` | 0.00% |
| **HTTP** | `Requests total` | 1,991,332 |
|           | `Requests per second` | 13,254.33 req/s |
|           | `Avg duration` | 113.84 ms |
|           | `Median duration` | 7.41 ms |
|           | `Min duration` | 47.71 µs |
|           | `Max duration` | 41.71 s |
|           | `p90 duration` | 447.17 ms |
|           | `p95 duration` | 485.8 ms |
|           | `p99 duration` | 674.41 ms |
| **Execution** | `Iteration avg duration` | 417.26 ms |
|               | `Iteration median` | 307.99 ms |
|               | `Iteration p90` | 750.18 ms |
|               | `Iteration p95` | 791.07 ms |
|               | `Iteration p99` | 1 s |
|               | `Iterations total` | 1,991,332 |
|               | `VUs active (avg)` | 369 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 406 MB (2.7 MB/s) |
|             | `Data sent` | 171 MB (1.1 MB/s) |

### 2. Django

| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 1m 0s ✗ |
|               | `http_req_failed` | 20.88% ✗ |
| **HTTP** | `Requests total` | 28,750 |
|           | `Requests per second` | 165.61 req/s |
|           | `Avg duration` | 33.37 s |
|           | `Median duration` | 36.31 s |
|           | `Min duration` | 3.59 ms |
|           | `Max duration` | 1m 0s |
|           | `p90 duration` | 1m 0s |
|           | `p95 duration` | 1m 0s |
|           | `p99 duration` | 1m 0s |
|           | `Expected response avg duration` | 26.34 s |
|           | `Expected response median` | 28 s |
|           | `Expected response p90` | 54.8 s |
|           | `Expected response p95` | 56.93 s |
|           | `Expected response p99` | 59.16 s |
|           | `HTTP failed` | 20.88% (6003 / 28750) |
| **Execution** | `Iteration avg duration` | 33.63 s |
|               | `Iteration median` | 36.74 s |
|               | `Iteration p90` | 1m 0s |
|               | `Iteration p95` | 1m 0s |
|               | `Iteration p99` | 1m 0s |
|               | `Iterations total` | 28,577 |
|               | `VUs active (avg)` | 163 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 7.5 MB (43 kB/s) |
|             | `Data sent` | 2.6 MB (15 kB/s) |

### 3. Golang

| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 100.01 ms ✓ |
|               | `http_req_failed` | 0.00% ✓ |
| **HTTP** | `Requests total` | 2,067,147 |
|           | `Requests per second` | 13,753.32 req/s |
|           | `Avg duration` | 24.54 ms |
|           | `Median duration` | 5.6 ms |
|           | `Min duration` | 56.18 µs |
|           | `Max duration` | 625.85 ms |
|           | `p90 duration` | 68.48 ms |
|           | `p95 duration` | 100.01 ms |
|           | `p99 duration` | 214.47 ms |
|           | `HTTP failed` | 0.00% (0 / 2,067,147) |
| **Execution** | `Iteration avg duration` | 366.73 ms |
|               | `Iteration median` | 314.57 ms |
|               | `Iteration p90` | 469.32 ms |
|               | `Iteration p95` | 559.39 ms |
|               | `Iteration p99` | 877.46 ms |
|               | `Iterations total` | 2,067,147 |
|               | `VUs active (avg)` | 304 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 323 MB (2.1 MB/s) |
|             | `Data sent` | 178 MB (1.2 MB/s) |
### 4. FastAPI

| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 3.25 s ✗ |
|               | `http_req_failed` | 0.00% ✓ |
| **HTTP** | `Requests total` | 462,322 |
|           | `Requests per second` | 3,076.34 req/s |
|           | `Avg duration` | 1.51 s |
|           | `Median duration` | 1.37 s |
|           | `Min duration` | 249.16 µs |
|           | `Max duration` | 6.36 s |
|           | `p90 duration` | 3.15 s |
|           | `p95 duration` | 3.25 s |
|           | `p99 duration` | 3.65 s |
|           | `HTTP failed` | 0.00% (0 / 462,322) |
| **Execution** | `Iteration avg duration` | 1.81 s |
|               | `Iteration median` | 1.67 s |
|               | `Iteration p90` | 3.45 s |
|               | `Iteration p95` | 3.55 s |
|               | `Iteration p99` | 3.95 s |
|               | `Iterations total` | 462,322 |
|               | `VUs active (avg)` | 359 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 74 MB (489 kB/s) |
|             | `Data sent` | 40 MB (265 kB/s) |

### 5. Rust (Axum)

| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 48 ms ✓ |
|               | `http_req_failed` | 0.00% ✓ |
| **HTTP** | `Requests total` | 2,473,482 |
|           | `Requests per second` | 16,459.03 req/s |
|           | `Avg duration` | 9.84 ms |
|           | `Median duration` | 941.84 µs |
|           | `Min duration` | 36.41 µs |
|           | `Max duration` | 589.41 ms |
|           | `p90 duration` | 25.93 ms |
|           | `p95 duration` | 48 ms |
|           | `p99 duration` | 141.42 ms |
|           | `HTTP failed` | 0.00% (0 / 2,473,482) |
| **Execution** | `Iteration avg duration` | 324.12 ms |
|               | `Iteration median` | 301.53 ms |
|               | `Iteration p90` | 373.11 ms |
|               | `Iteration p95` | 411.9 ms |
|               | `Iteration p99` | 668.22 ms |
|               | `Iterations total` | 2,473,482 |
|               | `VUs active (avg)` | 382 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 351 MB (2.3 MB/s) |
|             | `Data sent` | 213 MB (1.4 MB/s) |
### 6. Laravel 

| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 60.0 s ✗ |
|                | `http_req_failed` | 13.29% ✗ |
| **HTTP** | `Requests total` | 95,090 |
|           | `Requests per second` | 528.71 req/s |
|           | `Avg duration` | 9.51 s |
|           | `Median duration` | 224.65 ms |
|           | `Min duration` | 1.78 ms |
|           | `Max duration` | 60 s |
|           | `p90 duration` | 60.0 s |
|           | `p95 duration` | 60.0 s |
|           | `p99 duration` | 60.0 s |
|           | `HTTP failed` | 13.29% (12,645 / 95,090) |
| **Execution** | `Iteration avg duration` | 9.79 s |
|               | `Iteration median` | 525.23 ms |
|               | `Iteration p90` | 60.0 s |
|               | `Iteration p95` | 60.0 s |
|               | `Iteration p99` | 60.0 s |
|               | `Iterations total` | 95,039 |
|               | `VUs active (avg)` | 1 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 21 MB (116 kB/s) |
|             | `Data sent` | 8.2 MB (46 kB/s) |


## Json seriliazation / validation
### 1. FastAPI 

| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 4.74 s ✗ |
|               | `http_req_failed` | 0.00% ✓ |
| **HTTP** | `Requests total` | 357,114 |
|           | `Requests per second` | 2,376.31 req/s |
|           | `Avg duration` | 2.06 s |
|           | `Median duration` | 1.75 s |
|           | `Min duration` | 342.31 µs |
|           | `Max duration` | 10.41 s |
|           | `p90 duration` | 4.36 s |
|           | `p95 duration` | 4.74 s |
|           | `p99 duration` | 5.27 s |
|           | `HTTP failed` | 0.00% (0 / 357,114) |
| **Execution** | `Iteration avg duration` | 2.36 s |
|               | `Iteration median` | 2.06 s |
|               | `Iteration p90` | 4.66 s |
|               | `Iteration p95` | 5.04 s |
|               | `Iteration p99` | 5.57 s |
|               | `Iterations total` | 357,114 |
|               | `VUs active (avg)` | 379 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 99 MB (658 kB/s) |
|             | `Data sent` | 91 MB (608 kB/s) |
### 2. Golang

| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 186.04 ms ✓ |
|               | `http_req_failed` | 0.00% ✓ |
| **HTTP** | `Requests total` | 1,717,993 |
|           | `Requests per second` | 11,430.93 req/s |
|           | `Avg duration` | 41.92 ms |
|           | `Median duration` | 7.28 ms |
|           | `Min duration` | 61.4 µs |
|           | `Max duration` | 865.46 ms |
|           | `p90 duration` | 130.36 ms |
|           | `p95 duration` | 186.04 ms |
|           | `p99 duration` | 315.87 ms |
|           | `HTTP failed` | 0.00% (0 / 1,717,993) |
| **Execution** | `Iteration avg duration` | 414.06 ms |
|               | `Iteration median` | 320.27 ms |
|               | `Iteration p90` | 597.6 ms |
|               | `Iteration p95` | 715.78 ms |
|               | `Iteration p99` | 1.14 s |
|               | `Iterations total` | 1,717,993 |
|               | `VUs active (avg)` | 364 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 463 MB (3.1 MB/s) |
|             | `Data sent` | 440 MB (2.9 MB/s) |

### 3. NestJS (Fastify) 
| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 525.46 ms ✗ |
|               | `http_req_failed` | 0.36% ✓ |
| **HTTP** | `Requests total` | 1,269,341 |
|           | `Requests per second` | 7,649.92 req/s |
|           | `Avg duration` | 407.14 ms |
|           | `Median duration` | 56.8 ms |
|           | `Min duration` | 78.25 µs |
|           | `Max duration` | 60 s |
|           | `p90 duration` | 408.4 ms |
|           | `p95 duration` | 525.46 ms |
|           | `p99 duration` | 994.65 ms |
|           | `HTTP failed` | 0.36% (4,668 / 1,269,341) |
| **Execution** | `Iteration avg duration` | 706.07 ms |
|               | `Iteration median` | 357 ms |
|               | `Iteration p90` | 708.59 ms |
|               | `Iteration p95` | 825.54 ms |
|               | `Iteration p99` | 1.29 s |
|               | `Iterations total` | 1,269,313 |
|               | `VUs active (avg)` | 2 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 401 MB (2.4 MB/s) |
|             | `Data sent` | 325 MB (2.0 MB/s) |
### 4. Django 

| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 60.0 s ✗ |
|               | `http_req_failed` | 20.37% ✗ |
| **HTTP** | `Requests total` | 28,562 |
|           | `Requests per second` | 163.08 req/s |
|           | `Avg duration` | 33.15 s |
|           | `Median duration` | 36.11 s |
|           | `Min duration` | 4.06 ms |
|           | `Max duration` | 60 s |
|           | `p90 duration` | 60.0 s |
|           | `p95 duration` | 60.0 s |
|           | `p99 duration` | 60.0 s |
|           | `HTTP failed` | 20.37% (5,819 / 28,562) |
| **Execution** | `Iteration avg duration` | 33.42 s |
|               | `Iteration median` | 36.21 s |
|               | `Iteration p90` | 60.0 s |
|               | `Iteration p95` | 60.0 s |
|               | `Iteration p99` | 60.0 s |
|               | `Iterations total` | 28,521 |
|               | `VUs active (avg)` | 549 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 10 MB (59 kB/s) |
|             | `Data sent` | 7.8 MB (44 kB/s) |
### 5. Rust 

| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 85.11 ms ✓ |
|                | `http_req_failed` | 0.00% ✓ |
| **HTTP** | `Requests total` | 2,201,519 |
|           | `Requests per second` | 14,648.51 req/s |
|           | `Avg duration` | 18.89 ms |
|           | `Median duration` | 1.88 ms |
|           | `Min duration` | 43.88 µs |
|           | `Max duration` | 637.35 ms |
|           | `p90 duration` | 61.49 ms |
|           | `p95 duration` | 85.11 ms |
|           | `p99 duration` | 168.53 ms |
|           | `HTTP failed` | 0.00% (0 / 2,201,519) |
| **Execution** | `Iteration avg duration` | 350.38 ms |
|               | `Iteration median` | 303.4 ms |
|               | `Iteration p90` | 442.76 ms |
|               | `Iteration p95` | 502.09 ms |
|               | `Iteration p99` | 852.67 ms |
|               | `Iterations total` | 2,201,519 |
|               | `VUs active (avg)` | 321 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 558 MB (3.7 MB/s) |
|             | `Data sent` | 565 MB (3.8 MB/s) |

### 6. Laravel 
| **Category** | **Metric** | **Value** |
|---------------|-------------|-----------|
| **Thresholds** | `http_req_duration (p95)` | 60.0 s ✗ |
|                | `http_req_failed` | 15.42% ✗ |
| **HTTP** | `Requests total` | 83,706 |
|           | `Requests per second` | 466.88 req/s |
|           | `Avg duration` | 10.9 s |
|           | `Median duration` | 266.49 ms |
|           | `Min duration` | 1.9 ms |
|           | `Max duration` | 60 s |
|           | `p90 duration` | 60.0 s |
|           | `p95 duration` | 60.0 s |
|           | `p99 duration` | 60.0 s |
|           | `HTTP failed` | 15.42% (12,914 / 83,706) |
| **Execution** | `Iteration avg duration` | 11.18 s |
|               | `Iteration median` | 567.11 ms |
|               | `Iteration p90` | 60.0 s |
|               | `Iteration p95` | 60.0 s |
|               | `Iteration p99` | 60.0 s |
|               | `Iterations total` | 83,668 |
|               | `VUs active (avg)` | 3 |
|               | `VUs max` | 10,000 |
| **Network** | `Data received` | 25 MB (142 kB/s) |
|             | `Data sent` | 22 MB (120 kB/s) |`
