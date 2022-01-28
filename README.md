# Miller-Rabin Naughty Witness

So I saw [this video](https://www.youtube.com/watch?v=_MscGSN5J6o) from Numberphile, and decided to give answer brady's question about whether some witnesses are more likely to lie.
The program uses `rayon` to easily parallelise the search across all cores, as well as an implementation of the Miller-Rabin primality test adapted from [the wikipedia article](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).

## Results
| Limit   | Number | Lies told | Time taken for search |
|---------|--------|-----------|-----------------------|
| 100     | 38     | 4         | ~1.5ms                |
| 1000    | 64     | 16        | ~3.7ms                |
| 10000   | 512    | 68        | ~244.7ms              |
| 100000  | 4096   | 334       | ~36s                  |
| 1000000 | 4096   | 1163      | ~1.24 hours           |


It appears to be powers of two after 38, searching 1 million took so long that I don't think I fancy trying 10 million...
