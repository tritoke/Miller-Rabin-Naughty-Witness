# Miller-Rabin Naughty Witness

So I saw [this video](https://www.youtube.com/watch?v=_MscGSN5J6o) from Numberphile, and decided to give answer brady's question about whether some witnesses are more likely to lie.
The program uses `rayon` to easily parallelise the search across all cores, as well as an implementation of the Miller-Rabin primality test adapted from [the wikipedia article](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).

## Results
|Limit  | Number | Lies told |
|-------|--------|-----------|
|100    | 38     | 4         |
|1000   | 64     | 16        |
|10000  | 512    | 68        |
|100000 | 4096   | 334       |

