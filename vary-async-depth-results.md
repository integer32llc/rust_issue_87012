# Varying number of async functions called

| # of async fns | Item                                             | Self time | % of total time | Time     | Item count | Incremental result hashing time |
|---|--------------------------------------------------|-----------|-----------------|----------|------------|---------------------------------|
| 1 | evaluate_obligation                              | 20.58ms   | 11.782          | 21.58ms  | 1637       | 172.10µs                        |
| 2 | evaluate_obligation                              | 27.60ms   | 13.357          | 28.74ms  | 1727       | 225.46µs                        |
| 3 | evaluate_obligation                              | 34.50ms   | 18.234          | 35.59ms  | 1817       | 191.17µs                        |
| 4 | evaluate_obligation                              | 43.16ms   | 21.548          | 44.25ms  | 1907       | 225.69µs                        |
| 5 | evaluate_obligation                              | 52.59ms   | 24.419          | 53.70ms  | 1997       | 219.45µs                        |
| 6 | evaluate_obligation                              | 62.67ms   | 27.916          | 63.73ms  | 2087       | 215.59µs                        |
| 7 | evaluate_obligation                              | 73.32ms   | 30.837          | 74.47ms  | 2177       | 302.70µs                        |
| 8 | evaluate_obligation                              | 84.11ms   | 33.605          | 85.27ms  | 2267       | 230.18µs                        |
| 9 | evaluate_obligation                              | 99.17ms   | 36.662          | 100.60ms | 2357       | 968.78µs                        |
| 10 | evaluate_obligation                              | 112.16ms  | 39.570          | 113.53ms | 2447       | 327.75µs                        |
| 11 | evaluate_obligation                              | 125.94ms  | 41.832          | 127.35ms | 2537       | 313.84µs                        |
| 12 | evaluate_obligation                              | 140.61ms  | 43.858          | 141.98ms | 2627       | 421.63µs                        |
| 13 | evaluate_obligation                              | 156.13ms  | 47.091          | 157.59ms | 2717       | 308.85µs                        |
| 14 | evaluate_obligation                              | 174.22ms  | 49.291          | 175.71ms | 2807       | 534.84µs                        |
| 15 | evaluate_obligation                              | 191.78ms  | 51.522          | 193.31ms | 2897       | 679.09µs                        |
| 16 | evaluate_obligation                              | 209.62ms  | 53.411          | 211.22ms | 2987       | 1.01ms                          |
| 17 | evaluate_obligation                              | 227.41ms  | 55.136          | 229.10ms | 3077       | 421.92µs                        |
| 18 | evaluate_obligation                              | 247.01ms  | 57.091          | 248.56ms | 3167       | 389.70µs                        |
| 19 | evaluate_obligation                              | 268.32ms  | 58.703          | 269.86ms | 3257       | 485.01µs                        |
| 20 | evaluate_obligation                              | 289.99ms  | 60.321          | 291.74ms | 3347       | 411.64µs                        |
| 21 | evaluate_obligation                              | 310.91ms  | 62.113          | 312.58ms | 3437       | 419.14µs                        |
| 22 | evaluate_obligation                              | 335.29ms  | 63.640          | 337.01ms | 3527       | 476.92µs                        |
| 23 | evaluate_obligation                              | 361.93ms  | 64.926          | 363.66ms | 3617       | 1.26ms                          |
