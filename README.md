
## Go Redis Key value insert/s

```sh
# Linux:

time go run .
# go-redis-st1024
#  67.977s, 15_425 insert/s

# go-st1024
# 6.817s, 153_815 insert/s

# go-byte1024
# 3.176s, 330_136 insert/s

time cargo run --release
# rust--byte1024-array-fill
# 3s, 349_525 insert/s

# rust--byte1024 
# 10s, 104_857.6 insert/s

# rust-String
# 14s, 74_898 insert/s

# rust-byte1024-Uniform
# 14s, 74_898 insert/s
```

[How fast is Redis?](https://redis.io/topics/benchmarks)
```sh
sudo apt install redis-tools
redis-server

redis-benchmark -t set,lpush -n 100000 -q
# SET: 72_992.70 requests per second
# LPUSH: 74_794.31 requests per second
```