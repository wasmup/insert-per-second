package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {
	const n = 1024
	const max = n * n // 1_048_576
	m := make(map[[n]byte][n]byte, max)

	t0 := time.Now()
	for i := 0; i < max; i++ {
		buf := [n]byte{}
		rand.Read(buf[:])
		m[buf] = buf
	}
	s := time.Since(t0).Seconds()

	for _, v := range m {
		fmt.Println(v)
		break
	}
	fmt.Println("len=", len(m)) // 1_048_576

	fmt.Printf("%.3fs, %.0f insert/s\n", s, max/s)
	// 3.488s, 300_617 insert/s
}
