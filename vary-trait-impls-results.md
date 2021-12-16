# Varying number of trait implementations

| # of trait impls | Item                                             | Self time | % of total time | Time     | Item count | Incremental result hashing time |
|---|--------------------------------------------------|-----------|-----------------|----------|------------|---------------------------------|
| 0 | evaluate_obligation                              | 3.77ms    | 3.633           | 4.52ms   | 95         | 9.84µs                          |
| 1 | evaluate_obligation                              | 6.85ms    | 6.280           | 7.57ms   | 173        | 18.58µs                         |
| 2 | evaluate_obligation                              | 10.10ms   | 8.722           | 10.90ms  | 251        | 24.88µs                         |
| 3 | evaluate_obligation                              | 13.30ms   | 10.994          | 14.11ms  | 329        | 34.09µs                         |
| 4 | evaluate_obligation                              | 16.70ms   | 13.200          | 17.55ms  | 407        | 45.06µs                         |
| 5 | evaluate_obligation                              | 19.80ms   | 15.043          | 20.58ms  | 485        | 47.38µs                         |
| 6 | evaluate_obligation                              | 22.85ms   | 16.614          | 23.68ms  | 563        | 56.92µs                         |
| 7 | evaluate_obligation                              | 26.26ms   | 18.257          | 27.13ms  | 641        | 64.90µs                         |
| 8 | evaluate_obligation                              | 29.46ms   | 19.617          | 30.35ms  | 719        | 71.11µs                         |
| 9 | evaluate_obligation                              | 32.69ms   | 21.185          | 33.62ms  | 797        | 87.58µs                         |
| 10 | evaluate_obligation                              | 35.85ms   | 22.254          | 36.79ms  | 875        | 84.01µs                         |
| 11 | evaluate_obligation                              | 39.18ms   | 23.806          | 40.17ms  | 953        | 97.70µs                         |
| 12 | evaluate_obligation                              | 42.60ms   | 23.949          | 43.67ms  | 1031       | 114.56µs                        |
| 13 | evaluate_obligation                              | 45.48ms   | 26.066          | 46.47ms  | 1109       | 112.80µs                        |
| 14 | evaluate_obligation                              | 48.98ms   | 26.907          | 49.98ms  | 1187       | 117.56µs                        |
| 15 | evaluate_obligation                              | 51.85ms   | 27.422          | 52.87ms  | 1265       | 129.43µs                        |
| 16 | evaluate_obligation                              | 55.15ms   | 28.549          | 56.17ms  | 1343       | 130.49µs                        |
| 17 | evaluate_obligation                              | 58.31ms   | 29.436          | 59.35ms  | 1421       | 141.65µs                        |
| 18 | evaluate_obligation                              | 61.37ms   | 30.016          | 62.45ms  | 1499       | 155.46µs                        |
| 19 | evaluate_obligation                              | 64.90ms   | 31.152          | 66.02ms  | 1577       | 154.81µs                        |
| 20 | evaluate_obligation                              | 69.03ms   | 31.980          | 70.13ms  | 1655       | 201.99µs                        |
| 21 | evaluate_obligation                              | 72.81ms   | 32.173          | 74.02ms  | 1733       | 487.14µs                        |
| 22 | evaluate_obligation                              | 74.32ms   | 32.611          | 75.56ms  | 1811       | 185.04µs                        |
| 23 | evaluate_obligation                              | 77.82ms   | 33.527          | 78.96ms  | 1889       | 192.99µs                        |
| 24 | evaluate_obligation                              | 81.74ms   | 34.526          | 83.02ms  | 1967       | 273.52µs                        |
| 25 | evaluate_obligation                              | 84.29ms   | 34.611          | 85.50ms  | 2045       | 211.22µs                        |
| 26 | evaluate_obligation                              | 88.27ms   | 35.216          | 89.50ms  | 2123       | 232.60µs                        |
| 27 | evaluate_obligation                              | 90.57ms   | 34.779          | 91.67ms  | 2201       | 232.75µs                        |
| 28 | evaluate_obligation                              | 94.57ms   | 36.112          | 95.78ms  | 2279       | 240.66µs                        |
| 29 | evaluate_obligation                              | 97.07ms   | 36.136          | 98.39ms  | 2357       | 291.15µs                        |
| 30 | evaluate_obligation                              | 103.46ms  | 36.515          | 104.81ms | 2435       | 666.02µs                        |
