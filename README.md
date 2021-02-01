# HTTP ascii POC

An experiment on the https://github.com/hyperium/http crate.
On this branch I experiment with using the ascii crate instead of the custom `ByteStr` found in the baseline crate.
Once I know that a data is an ascii string I can leverage that fact to drop certain runtime checks, to see how that would impact performance.

It doesn't make much sense to use this as is at this point.
I dropped potentially necessary safety in favour of unsafe code and added unnecessary overhead at other places.
You have been warned.

# Bench results

Current regressions
|  name                                                  |  baseline_latest.bench ns/iter|  ascii_latest.bench ns/iter|  diff ns/iter |  diff %|  speedup  |
| -------------------------------------------------------|-------------------------------|----------------------------|---------------|--------|---------- |
|  basic::get_100_std::header_map                        |  538                          |  751                       |           213 |  39.59%|   x 0.72  |
|  basic::get_100_std::order_map_fnv                     |  695                          |  704                       |             9 |   1.29%|   x 0.99  |
|  basic::get_100_std::order_map_seahash                 |  1,464                        |  1,531                     |            67 |   4.58%|   x 0.96  |
|  basic::get_10_custom_short::header_map                |  131                          |  177                       |            46 |  35.11%|   x 0.74  |
|  basic::get_10_custom_short::order_map_fnv             |  151                          |  323                       |           172 | 113.91%|   x 0.47  |
|  basic::get_10_custom_short::vec_map                   |  233                          |  274                       |            41 |  17.60%|   x 0.85  |
|  basic::get_10_of_20_std::header_map                   |  48                           |  65                        |            17 |  35.42%|   x 0.74  |
|  basic::get_10_of_20_std::order_map_fnv                |  54                           |  71                        |            17 |  31.48%|   x 0.76  |
|  basic::get_10_of_20_std::order_map_seahash            |  138                          |  146                       |             8 |   5.80%|   x 0.95  |
|  basic::hn_hdrs_set_11_get_with_miss::header_map       |  563                          |  570                       |             7 |   1.24%|   x 0.99  |
|  basic::hn_hdrs_set_11_get_with_miss::order_map_fnv    |  607                          |  656                       |            49 |   8.07%|   x 0.93  |
|  basic::hn_hdrs_set_11_get_with_miss::order_map_seahash|  1,014                        |  1,061                     |            47 |   4.64%|   x 0.96  |
|  basic::hn_hdrs_set_8_get_many::header_map             |  417                          |  454                       |            37 |   8.87%|   x 0.92  |
|  basic::hn_hdrs_set_8_get_many::order_map_fnv          |  508                          |  545                       |            37 |   7.28%|   x 0.93  |
|  basic::hn_hdrs_set_8_get_many::order_map_seahash      |  824                          |  840                       |            16 |   1.94%|   x 0.98  |
|  basic::hn_hdrs_set_8_get_many::vec_map                |  255                          |  291                       |            36 |  14.12%|   x 0.88  |
|  basic::hn_hdrs_set_8_get_miss::header_map             |  276                          |  289                       |            13 |   4.71%|   x 0.96  |
|  basic::hn_hdrs_set_8_get_miss::order_map_fnv          |  393                          |  397                       |             4 |   1.02%|   x 0.99  |
|  basic::hn_hdrs_set_8_get_miss::order_map_seahash      |  500                          |  502                       |             2 |   0.40%|   x 1.00  |
|  basic::hn_hdrs_set_8_get_miss::vec_map                |  132                          |  153                       |            21 |  15.91%|   x 0.86  |
|  basic::insert_100_custom_headers::header_map          |  5,701                        |  6,496                     |           795 |  13.94%|   x 0.88  |
|  basic::insert_100_custom_headers::order_map_fnv       |  8,812                        |  11,800                    |         2,988 |  33.91%|   x 0.75  |
|  basic::insert_100_custom_headers::order_map_seahash   |  7,485                        |  8,217                     |           732 |   9.78%|   x 0.91  |
|  basic::insert_100_custom_headers::vec_map             |  18,945                       |  27,928                    |         8,983 |  47.42%|   x 0.68  |
|  basic::insert_4_std_get_30::header_map                |  254                          |  301                       |            47 |  18.50%|   x 0.84  |
|  basic::insert_4_std_get_30::order_map_fnv             |  402                          |  417                       |            15 |   3.73%|   x 0.96  |
|  basic::insert_4_std_get_30::order_map_seahash         |  654                          |  675                       |            21 |   3.21%|   x 0.97  |
|  basic::insert_4_std_get_30::vec_map                   |  115                          |  121                       |             6 |   5.22%|   x 0.95  |
|  basic::insert_500_custom_headers::header_map          |  29,277                       |  33,074                    |         3,797 |  12.97%|   x 0.89  |
|  basic::insert_500_custom_headers::order_map_fnv       |  32,286                       |  45,390                    |        13,104 |  40.59%|   x 0.71  |
|  basic::insert_500_custom_headers::order_map_seahash   |  38,567                       |  41,129                    |         2,562 |   6.64%|   x 0.94  |
|  basic::insert_500_custom_headers::vec_map             |  406,077                      |  518,039                   |       111,962 |  27.57%|   x 0.78  |
|  basic::insert_6_std_get_6::header_map                 |  169                          |  191                       |            22 |  13.02%|   x 0.88  |
|  basic::insert_6_std_get_6::order_map_fnv              |  305                          |  389                       |            84 |  27.54%|   x 0.78  |
|  basic::insert_6_std_get_6::order_map_seahash          |  401                          |  402                       |             1 |   0.25%|   x 1.00  |
|  basic::insert_79_custom_std_headers::header_map       |  4,847                        |  5,211                     |           364 |   7.51%|   x 0.93  |
|  basic::insert_79_custom_std_headers::order_map_fnv    |  3,953                        |  5,443                     |         1,490 |  37.69%|   x 0.73  |
|  basic::insert_79_custom_std_headers::order_map_seahash|  6,035                        |  6,518                     |           483 |   8.00%|   x 0.93  |
|  basic::insert_all_std_headers::order_map_fnv          |  2,939                        |  3,057                     |           118 |   4.01%|   x 0.96  |
|  basic::insert_one_100_char_header::order_map_fnv      |  147                          |  158                       |            11 |   7.48%|   x 0.93  |
|  basic::insert_one_100_char_header::order_map_seahash  |  100                          |  106                       |             6 |   6.00%|   x 0.94  |
|  basic::insert_one_100_char_header::vec_map            |  37                           |  38                        |             1 |   2.70%|   x 0.97  |
|  basic::insert_one_15_char_header::order_map_fnv       |  68                           |  80                        |            12 |  17.65%|   x 0.85  |
|  basic::insert_one_15_char_header::vec_map             |  35                           |  36                        |             1 |   2.86%|   x 0.97  |
|  basic::insert_one_25_char_header::order_map_fnv       |  79                           |  89                        |            10 |  12.66%|   x 0.89  |
|  basic::insert_one_25_char_header::order_map_seahash   |  94                           |  96                        |             2 |   2.13%|   x 0.98  |
|  basic::insert_one_50_char_header::order_map_fnv       |  100                          |  111                       |            11 |  11.00%|   x 0.90  |
|  basic::insert_one_50_char_header::order_map_seahash   |  96                           |  99                        |             3 |   3.12%|   x 0.97  |
|  basic::insert_one_50_char_header::vec_map             |  35                           |  36                        |             1 |   2.86%|   x 0.97  |
|  basic::new_insert_get_host::header_map                |  45                           |  50                        |             5 |  11.11%|   x 0.90  |
|  basic::new_insert_get_host::order_map_fnv             |  56                           |  58                        |             2 |   3.57%|   x 0.97  |
|  basic::set_10_get_1_custom_long::order_map_fnv        |  980                          |  1,487                     |           507 |  51.73%|   x 0.66  |
|  basic::set_10_get_1_custom_long::vec_map              |  531                          |  1,010                     |           479 |  90.21%|   x 0.53  |
|  basic::set_10_get_1_custom_med::order_map_fnv         |  838                          |  1,102                     |           264 |  31.50%|   x 0.76  |
|  basic::set_10_get_1_custom_med::order_map_seahash     |  837                          |  838                       |             1 |   0.12%|   x 1.00  |
|  basic::set_10_get_1_custom_med::vec_map               |  485                          |  689                       |           204 |  42.06%|   x 0.70  |
|  basic::set_10_get_1_custom_short::order_map_fnv       |  797                          |  1,014                     |           217 |  27.23%|   x 0.79  |
|  basic::set_10_get_1_custom_short::order_map_seahash   |  837                          |  878                       |            41 |   4.90%|   x 0.95  |
|  basic::set_10_get_1_custom_very_long::order_map_fnv   |  1,199                        |  2,051                     |           852 |  71.06%|   x 0.58  |
|  basic::set_10_get_1_custom_very_long::vec_map         |  529                          |  1,448                     |           919 | 173.72%|   x 0.37  |
|  basic::set_20_get_1_custom_long::order_map_fnv        |  1,749                        |  2,637                     |           888 |  50.77%|   x 0.66  |
|  basic::set_20_get_1_custom_long::vec_map              |  1,155                        |  2,009                     |           854 |  73.94%|   x 0.57  |
|  basic::set_20_get_1_custom_med::header_map            |  1,228                        |  1,289                     |            61 |   4.97%|   x 0.95  |
|  basic::set_20_get_1_custom_med::order_map_fnv         |  1,457                        |  2,132                     |           675 |  46.33%|   x 0.68  |
|  basic::set_20_get_1_custom_med::order_map_seahash     |  1,609                        |  1,741                     |           132 |   8.20%|   x 0.92  |
|  basic::set_20_get_1_custom_med::vec_map               |  1,086                        |  1,631                     |           545 |  50.18%|   x 0.67  |
|  basic::set_20_get_1_custom_short::header_map          |  1,115                        |  1,155                     |            40 |   3.59%|   x 0.97  |
|  basic::set_20_get_1_custom_short::order_map_fnv       |  1,379                        |  1,873                     |           494 |  35.82%|   x 0.74  |
|  basic::set_20_get_1_custom_short::order_map_seahash   |  1,581                        |  1,690                     |           109 |   6.89%|   x 0.94  |
|  basic::set_20_get_1_custom_short::vec_map             |  1,136                        |  1,258                     |           122 |  10.74%|   x 0.90  |
|  basic::set_20_get_1_custom_very_long::order_map_fnv   |  2,189                        |  3,621                     |         1,432 |  65.42%|   x 0.60  |
|  basic::set_20_get_1_custom_very_long::vec_map         |  1,145                        |  3,073                     |         1,928 | 168.38%|   x 0.37  |
|  basic::set_20_get_1_std::header_map                   |  620                          |  627                       |             7 |   1.13%|   x 0.99  |
|  basic::set_20_get_1_std::order_map_seahash            |  1,031                        |  1,056                     |            25 |   2.42%|   x 0.98  |
|  basic::set_20_get_1_std::vec_map                      |  375                          |  403                       |            28 |   7.47%|   x 0.93  |
|  basic::set_8_get_1_std::header_map                    |  266                          |  291                       |            25 |   9.40%|   x 0.91  |
|  basic::set_8_get_1_std::order_map_fnv                 |  428                          |  450                       |            22 |   5.14%|   x 0.95  |
|  basic::set_8_get_1_std::order_map_seahash             |  488                          |  506                       |            18 |   3.69%|   x 0.96  |
|  basic::set_8_get_1_std::vec_map                       |  135                          |  141                       |             6 |   4.44%|   x 0.96  |
|  header_name_various                                   |  2,879                        |  3,334                     |           455 |  15.80%|   x 0.86  |
|  uri_parse_relative_medium                             |  79 (759 MB/s)                |  88 (681 MB/s)             |             9 |  11.39%|   x 0.90  |
|  uri_parse_relative_query                              |  107 (785 MB/s)               |  110 (763 MB/s)            |             3 |   2.80%|   x 0.97  |
|  uri_parse_slash                                       |  19 (52 MB/s)                 |  33 (30 MB/s)              |            14 |  73.68%|   x 0.58  |

Current improvements
|  name                                                   |  baseline_latest.bench ns/iter|  ascii_latest.bench ns/iter|  diff ns/iter|   diff %|  speedup  |
| --------------------------------------------------------|-------------------------------|----------------------------|--------------|---------|---------- |
|  basic::get_100_std::vec_map                            |  1,871                        |  1,527                     |          -344|  -18.39%|   x 1.23  |
|  basic::get_10_custom_short::order_map_seahash          |  341                          |  330                       |           -11|   -3.23%|   x 1.03  |
|  basic::get_10_of_20_std::vec_map                       |  43                           |  32                        |           -11|  -25.58%|   x 1.34  |
|  basic::hn_hdrs_set_11_get_with_miss::vec_map           |  380                          |  355                       |           -25|   -6.58%|   x 1.07  |
|  basic::insert_6_std_get_6::vec_map                     |  141                          |  138                       |            -3|   -2.13%|   x 1.02  |
|  basic::insert_79_custom_std_headers::vec_map           |  8,037                        |  4,251                     |        -3,786|  -47.11%|   x 1.89  |
|  basic::insert_all_std_headers::header_map              |  2,119                        |  2,072                     |           -47|   -2.22%|   x 1.02  |
|  basic::insert_all_std_headers::order_map_seahash       |  3,577                        |  3,432                     |          -145|   -4.05%|   x 1.04  |
|  basic::insert_all_std_headers::vec_map                 |  2,397                        |  1,825                     |          -572|  -23.86%|   x 1.31  |
|  basic::insert_one_100_char_header::header_map          |  142                          |  141                       |            -1|   -0.70%|   x 1.01  |
|  basic::insert_one_15_char_header::header_map           |  65                           |  63                        |            -2|   -3.08%|   x 1.03  |
|  basic::insert_one_15_char_header::order_map_seahash    |  93                           |  93                        |             0|    0.00%|   x 1.00  |
|  basic::insert_one_25_char_header::header_map           |  74                           |  72                        |            -2|   -2.70%|   x 1.03  |
|  basic::insert_one_25_char_header::vec_map              |  39                           |  36                        |            -3|   -7.69%|   x 1.08  |
|  basic::insert_one_50_char_header::header_map           |  96                           |  94                        |            -2|   -2.08%|   x 1.02  |
|  basic::new_insert_get_host::order_map_seahash          |  83                           |  80                        |            -3|   -3.61%|   x 1.04  |
|  basic::new_insert_get_host::vec_map                    |  17                           |  17                        |             0|    0.00%|   x 1.00  |
|  basic::set_10_get_1_custom_long::header_map            |  756                          |  729                       |           -27|   -3.57%|   x 1.04  |
|  basic::set_10_get_1_custom_long::order_map_seahash     |  866                          |  859                       |            -7|   -0.81%|   x 1.01  |
|  basic::set_10_get_1_custom_med::header_map             |  614                          |  583                       |           -31|   -5.05%|   x 1.05  |
|  basic::set_10_get_1_custom_short::header_map           |  561                          |  539                       |           -22|   -3.92%|   x 1.04  |
|  basic::set_10_get_1_custom_short::vec_map              |  534                          |  529                       |            -5|   -0.94%|   x 1.01  |
|  basic::set_10_get_1_custom_very_long::header_map       |  982                          |  956                       |           -26|   -2.65%|   x 1.03  |
|  basic::set_10_get_1_custom_very_long::order_map_seahash|  885                          |  885                       |             0|    0.00%|   x 1.00  |
|  basic::set_10_get_1_std::header_map                    |  332                          |  331                       |            -1|   -0.30%|   x 1.00  |
|  basic::set_10_get_1_std::order_map_fnv                 |  489                          |  465                       |           -24|   -4.91%|   x 1.05  |
|  basic::set_10_get_1_std::order_map_seahash             |  570                          |  568                       |            -2|   -0.35%|   x 1.00  |
|  basic::set_10_get_1_std::vec_map                       |  217                          |  207                       |           -10|   -4.61%|   x 1.05  |
|  basic::set_20_get_1_custom_long::header_map            |  1,537                        |  1,456                     |           -81|   -5.27%|   x 1.06  |
|  basic::set_20_get_1_custom_long::order_map_seahash     |  1,634                        |  1,613                     |           -21|   -1.29%|   x 1.01  |
|  basic::set_20_get_1_custom_very_long::header_map       |  1,945                        |  1,850                     |           -95|   -4.88%|   x 1.05  |
|  basic::set_20_get_1_custom_very_long::order_map_seahash|  1,710                        |  1,629                     |           -81|   -4.74%|   x 1.05  |
|  basic::set_20_get_1_std::order_map_fnv                 |  856                          |  848                       |            -8|   -0.93%|   x 1.01  |
|  from_shared_long                                       |  93 (1215 MB/s)               |  93 (1215 MB/s)            |             0|    0.00%|   x 1.00  |
|  from_shared_short                                      |  22 (409 MB/s)                |  22 (409 MB/s)             |             0|    0.00%|   x 1.00  |
|  from_shared_unchecked_long                             |  12 (9416 MB/s)               |  12 (9416 MB/s)            |             0|    0.00%|   x 1.00  |
|  from_shared_unchecked_short                            |  12 (750 MB/s)                |  12 (750 MB/s)             |             0|    0.00%|   x 1.00  |
|  header_name_bad                                        |  10                           |  10                        |             0|    0.00%|   x 1.00  |
|  header_name_easy                                       |  14                           |  14                        |             0|    0.00%|   x 1.00  |
|  method_easy                                            |  2                            |  2                         |             0|    0.00%|   x 1.00  |
|  method_various                                         |  73                           |  69                        |            -4|   -5.48%|   x 1.06  |

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
