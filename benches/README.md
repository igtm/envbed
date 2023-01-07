
```sh
$ cargo criterion

replace_dollar_braces_process                                                                             
                        time:   [5.6876 µs 5.7222 µs 5.7640 µs]
                        change: [+0.1295% +0.7924% +1.4137%] (p = 0.02 < 0.05)
                        Change within noise threshold.

replace_dollar_braces_with_hashmap_process                                                                             
                        time:   [2.8887 µs 2.9007 µs 2.9133 µs]
                        change: [-1.7636% -1.3424% -0.8391%] (p = 0.00 < 0.05)
                        Change within noise threshold.

replace_double_braces_process                                                                             
                        time:   [3.2507 µs 3.2640 µs 3.2793 µs]
                        change: [+0.4041% +0.6562% +0.9007%] (p = 0.00 < 0.05)
                        Change within noise threshold.

replace_double_braces_with_hashmap                                                                             
                        time:   [1.9008 µs 1.9135 µs 1.9273 µs]
                        change: [+1.2092% +2.0501% +2.9465%] (p = 0.00 < 0.05)
                        Performance has regressed.
```