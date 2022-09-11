package main

import (
	"fmt"
	"strings"
)

func convert(c byte) byte {
	if c == 'A' {
		return 'C'
	} else if c == 'C' {
		return 'G'
	} else if c == 'G' {
		return 'T'
	} else {
		return 'A'
	}
}

func replaceAtIndex(input string, replacement byte, index int) string {
    return strings.Join([]string{input[:index], string(replacement), input[index+1:]}, "")
}

func main() {
	fmt.Println("Start")
	
	opt := "ACGT"
	s := ""
	s_last := ""
	len_str := 13
	change_next := false

	for i := 0; i < len_str; i++ {
		s += string(opt[0])
	}

	for i := 0; i < len_str; i++ {
		s_last += string(opt[len(opt) - 1:])
	}

	counter := 1
	for s != s_last {
		counter++

		change_next = true
		for i := 0; i < len_str; i++ {
			if change_next {
				if string(s[i]) == opt[len(opt) - 1:] {
					s = replaceAtIndex(s, convert(s[i]), i)
					change_next = true
				} else {
					s = replaceAtIndex(s, convert(s[i]), i)
					break
				}
			}
		}
	}

	fmt.Println("Number of generated k-mers:", counter)
	fmt.Println("Finish!")
}