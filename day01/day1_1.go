package main

import (
	"fmt"
	"io"
	"log"
	"os"
)

func calculateFuel(v int64) int64 {
	return (v / 3) - 2
}

func main() {
	var value int64
	var sum int64
	for {
		_, err := fmt.Fscanf(os.Stdin, "%d", &value)
		if err != nil {
			if err != io.EOF {
				log.Fatalf("Error reading from file: %v\n", err)
			}
			break
		}
		sum += calculateFuel(value)
	}
	fmt.Printf("Sum is: %v\n", sum)

}
