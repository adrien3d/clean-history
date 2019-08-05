package main

import (
	"fmt"
	"github.com/adrien3d/utils"
	"io/ioutil"
	"os"
	"sort"
	"strings"
)

func OpenHistory(fileName string) (ret [][]string, len int) {
	content, err := ioutil.ReadFile(fileName)
	utils.CheckErr(err)

	lines := strings.Split(string(content), "\n")

	for _, line := range lines {
		lineSplit := strings.Split(line, ";")
		if cap(lineSplit) > 1 {
			ret = append(ret, lineSplit)
			len++
		}
	}

	return ret, len
}

func SaveHistory(fileName string, data[][]string)(len int) {
	f, err := os.Create(fileName)
	utils.CheckErr(err)
	defer f.Close()

	for _, elt := range data {
		_, err := f.WriteString(elt[0] + ";" + elt[1] + "\n")
		utils.CheckErr(err)
		len++
	}
	return len
}

func main() {
	res, orSize := OpenHistory("/Users/adrien/.zsh_history")
	// Order by command alphabetical order
	// fmt.Println(res)
	sort.Slice(res, func(i, j int) bool { return res[i][1] < res[j][1]})

	// Remove duplicates
	for r := 0 ; r < 100; r++ {
		for i := 0; i < len(res) - 1; i++ {
			if res[i][1] == res[i+1][1] {
				res = append(res[:i], res[i+1:]...)
			}
		}
	}
	sort.Slice(res, func(i, j int) bool { return res[i][0] < res[j][0]})

	destSize := SaveHistory("/Users/adrien/.zsh_history", res)

	fmt.Println("Sizes: Original:", orSize, "\tRemoved:", orSize-destSize, "\tNew:", destSize)
}
