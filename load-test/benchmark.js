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
  const res = http.get(`${BASE_URL}/api/v1/greetings`);
  check(res, {
    'status is 200': (r) => r.status === 200,
  });
  sleep(0.3);
}
