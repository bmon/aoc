package main

import (
	"crypto/md5"
	"encoding/hex"
	"io/ioutil"
	"log"
	"os"
	"strconv"
)

func main() {
	input, err := ioutil.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	input = input[:len(input)-1]
	findMd5Sum(input, "000000")
}

func findMd5Sum(secret []byte, target string) int {
	hasher := md5.New()
	for i := 0; ; i++ {
		data := append(secret, []byte(strconv.Itoa(i))...)
		hasher.Reset()
		hasher.Write(data)
		res := hex.EncodeToString(hasher.Sum(nil))
		if res[:6] == target {
			log.Println(i, res)
			return i
		}
	}
}
