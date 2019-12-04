package main

import (
	"fmt"
	"io"
	"log"
	"os"
)

func calculateFuel(v int64) int64 {
	if v/3-2 <= 0 {
		return 0
	}
	fuel := (v / 3) - 2
	fmt.Printf("fuel: %v\n", fuel)
	return fuel + calculateFuel(fuel)
}

func main() {
	var module int64
	var sum int64
	for {
		_, err := fmt.Fscanf(os.Stdin, "%d", &module)
		if err != nil {
			if err != io.EOF {
				log.Fatalf("Error reading from file: %v\n", err)
			}
			break
		}
		sum += calculateFuel(module)
	}
	fmt.Printf("Sum is: %v\n", sum)

}
