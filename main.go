package main

import (
	"log"
	"math/big"
	"net/http"
	"strconv"
	"strings"
)

func factorial(n int64) *big.Int {

	if n < 0 {
		return nil
	}

	r := big.NewInt(1)
	var f big.Int
	for i := int64(2); i <= n; i++ {
		r.Mul(r, f.SetInt64(i))
	}

	return r

}
func sayHello(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello, World"))
}

func factorial_handler(w http.ResponseWriter, r *http.Request) {
	number, err :=
		strconv.ParseInt(strings.TrimPrefix(r.URL.Path, "/factorial/"), 10, 64)

	if err != nil {
		log.Fatal(err)
	}

	w.Write([]byte(factorial(number).String()))
}

func main() {
	http.HandleFunc("/", sayHello)
	http.HandleFunc("/factorial/", factorial_handler)
	if err := http.ListenAndServe(":8080", nil); err != nil {
		panic(err)
	}
}
