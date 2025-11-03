import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  stages: [
    { duration: '20s', target: 1000 },   // Ramp-up to 1000 users
    { duration: '40s', target: 5000 },   // Ramp-up to 5000 users
    { duration: '40s', target: 10000 },  // Ramp-up to 10k users
    { duration: '30s', target: 10000 },  // Stay at 10k users for 30 seconds
    { duration: '20s', target: 0 },      // Ramp-down to 0 users
  ],
  thresholds: {
    'http_req_duration': ['p(95)<500'], // 95% of requests must complete below 500ms
    'http_req_failed': ['rate<0.01'],   // http errors should be less than 1%
  },
  summaryTrendStats: ['avg', 'min', 'med', 'max', 'p(90)', 'p(95)', 'p(99)', 'count'],
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

export default function () {
  const payload = {
    id: Math.floor(Math.random() * 1e9),
    name: `user-${__VU}-${__ITER}`,
    message: `hello from vu ${__VU} iter ${__ITER}`,
    greetDate: new Date().toISOString(),
  };

  const headers = { 'Content-Type': 'application/json' };
  const res = http.post(`${BASE_URL}/api/v1/greetings`, JSON.stringify(payload), { headers });

  check(res, {
    'status is 2xx': (r) => r.status >= 200 && r.status < 300,
  });

    if (res.status >= 400) {
    console.error(`Error response [${res.status}]: ${res.body}`);
  }


  sleep(0.3);
}
