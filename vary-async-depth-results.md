# Varying number of async functions called

| # of async fns | Item                                             | Self time | % of total time | Time     | Item count | Incremental result hashing time |
|---|--------------------------------------------------|-----------|-----------------|----------|------------|---------------------------------|
| 1 | evaluate_obligation                              | 20.46ms   | 11.711          | 21.47ms  | 1637       | 160.63µs                        |
| 2 | evaluate_obligation                              | 27.35ms   | 14.758          | 28.38ms  | 1727       | 223.08µs                        |
| 3 | evaluate_obligation                              | 34.76ms   | 18.239          | 35.85ms  | 1817       | 185.94µs                        |
| 4 | evaluate_obligation                              | 43.54ms   | 21.304          | 44.73ms  | 1907       | 283.39µs                        |
| 5 | evaluate_obligation                              | 52.97ms   | 24.537          | 54.06ms  | 1997       | 225.41µs                        |
| 6 | evaluate_obligation                              | 62.44ms   | 27.921          | 63.50ms  | 2087       | 227.20µs                        |
| 7 | evaluate_obligation                              | 73.73ms   | 30.590          | 74.94ms  | 2177       | 283.14µs                        |
| 8 | evaluate_obligation                              | 86.41ms   | 32.629          | 87.63ms  | 2267       | 321.48µs                        |
| 9 | evaluate_obligation                              | 103.08ms  | 35.044          | 104.65ms | 2357       | 847.54µs                        |
| 10 | evaluate_obligation                              | 117.10ms  | 37.947          | 118.69ms | 2447       | 693.05µs                        |
| 11 | evaluate_obligation                              | 128.88ms  | 42.201          | 130.36ms | 2537       | 922.60µs                        |
| 12 | evaluate_obligation                              | 148.34ms  | 43.024          | 150.03ms | 2627       | 995.03µs                        |
| 13 | evaluate_obligation                              | 164.04ms  | 44.330          | 165.82ms | 2717       | 920.25µs                        |
| 14 | evaluate_obligation                              | 181.43ms  | 48.584          | 183.16ms | 2807       | 1.17ms                          |
| 15 | evaluate_obligation                              | 204.16ms  | 49.517          | 206.12ms | 2897       | 1.34ms                          |
| 16 | evaluate_obligation                              | 220.80ms  | 53.307          | 222.82ms | 2987       | 1.40ms                          |
| 17 | evaluate_obligation                              | 241.20ms  | 53.614          | 243.34ms | 3077       | 1.33ms                          |
| 18 | evaluate_obligation                              | 251.76ms  | 57.032          | 253.73ms | 3167       | 1.22ms                          |
| 19 | evaluate_obligation                              | 274.58ms  | 58.271          | 276.36ms | 3257       | 1.43ms                          |
| 20 | evaluate_obligation                              | 301.21ms  | 59.511          | 303.37ms | 3347       | 1.25ms                          |
| 21 | evaluate_obligation                              | 328.34ms  | 60.511          | 330.53ms | 3437       | 1.93ms                          |
| 22 | evaluate_obligation                              | 354.24ms  | 61.547          | 356.54ms | 3527       | 1.42ms                          |
