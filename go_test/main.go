package main

import (
	"fmt"
	"time"

	"example.com/benchmark"
)

func main() {
	fibRecuStart := time.Now()
	fibRecuRes := benchmark.FibRecu(40)
	fibRecuEnd := time.Now()
	fmt.Println("递归执行时间:", fibRecuEnd.Sub(fibRecuStart))
	fmt.Println("递归执行结果:", fibRecuRes)

	fibIterStart := time.Now()
	fibIterRes := benchmark.FibIter(40)
	fibIterEnd := time.Now()
	fmt.Println("迭代执行时间:", fibIterEnd.Sub(fibIterStart))
	fmt.Println("迭代执行结果:", fibIterRes)
}
