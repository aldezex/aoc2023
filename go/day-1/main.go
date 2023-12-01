package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
    file, err := os.Open("./calibration-document.txt")
    if err != nil {
        fmt.Println(err)
    }
    defer file.Close()

    scanner := bufio.NewScanner(file)

    var total int = 0
    for scanner.Scan() {
        var digitsFound []int
        line := scanner.Text()

        for _, digit := range line {
            if len(digitsFound) == 2 {
                break
            }

            number, err := strconv.Atoi(string(digit))

            fmt.Println(number)
            fmt.Println(err)
        }
        
        fmt.Println(digitsFound)

        stringNumber := strconv.Itoa(digitsFound[0]) + strconv.Itoa(digitsFound[1])
        number, err := strconv.Atoi(stringNumber)

        if err != nil {
            fmt.Println(err)
        }

        total += number
    }

    if err := scanner.Err(); err != nil {
        fmt.Println(err)
    }
}
