
## Run
```sh
$ time go build -ldflags=-s .
# real    0m0.361s
# user    0m0.015s
# sys     0m0.046s

# Run/Output:
# len= 1048576
# 3.417s, 306828 insert/s

# real    0m3.736s
# user    0m0.000s
# sys     0m0.031s

$ time go run .
# len= 1048576
# 3.557s, 294_793 insert/s

# real    0m4.359s
# user    0m0.046s
# sys     0m0.015s
```
