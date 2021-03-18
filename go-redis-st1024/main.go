package main

import (
	"context"
	"fmt"
	"log"
	"math/rand"
	"time"
	"unsafe"

	"github.com/go-redis/redis/v8"
)

func main() {
	var ctx = context.Background()
	rdb := redis.NewClient(&redis.Options{
		Addr:     "127.0.0.1:6379",
		Password: "", // no password set
		DB:       0,  // use default DB
	})

	const n = 1024
	const max = n * n // 1_048_576
	var key string

	t0 := time.Now()
	for i := 0; i < max; i++ {
		key = RandStringBytesMaskImprSrcUnsafe(n)
		err := rdb.Set(ctx, key, key, 0).Err() // value=key
		if err != nil {
			log.Fatal(err)
		}
	}
	s := time.Since(t0).Seconds()

	val, err := rdb.Get(ctx, key).Result()
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("key", key, val)

	fmt.Printf("%.3fs, %.0f insert/s\n", s, max/s)
	// 67.977s, 15_425 insert/s
}

// https://stackoverflow.com/questions/22892120/how-to-generate-a-random-string-of-a-fixed-length-in-go
func RandStringBytesMaskImprSrcUnsafe(n int) string {
	b := make([]byte, n)
	// A src.Int63() generates 63 random bits, enough for letterIdxMax characters!
	for i, cache, remain := n-1, src.Int63(), letterIdxMax; i >= 0; {
		if remain == 0 {
			cache, remain = src.Int63(), letterIdxMax
		}
		if idx := int(cache & letterIdxMask); idx < len(letterBytes) {
			b[i] = letterBytes[idx]
			i--
		}
		cache >>= letterIdxBits
		remain--
	}

	return *(*string)(unsafe.Pointer(&b))
}

var src = rand.NewSource(time.Now().UnixNano())

const letterBytes = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
const (
	letterIdxBits = 6                    // 6 bits to represent a letter index
	letterIdxMask = 1<<letterIdxBits - 1 // All 1-bits, as many as letterIdxBits
	letterIdxMax  = 63 / letterIdxBits   // # of letter indices fitting in 63 bits
)
