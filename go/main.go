
package main

import (
    "fmt"
    "net/http"
    "strconv"
    "encoding/json"
)

type Response struct {
    Message string `json:"message,omitempty"`
    Sequence []int `json:"sequence,omitempty"`
}

func helloHandler(w http.ResponseWriter, r *http.Request) {
    json.NewEncoder(w).Encode(Response{Message: "Hello from Go!"})
}

func fibonacciHandler(w http.ResponseWriter, r *http.Request) {
    nStr := r.URL.Query().Get("n")
    n, _ := strconv.Atoi(nStr)
    if n == 0 {
        n = 10
    }
    seq := []int{}
    a, b := 0, 1
    for i := 0; i < n; i++ {
        seq = append(seq, a)
        a, b = b, a+b
    }
    json.NewEncoder(w).Encode(Response{Sequence: seq})
}

func main() {
    http.HandleFunc("/hello", helloHandler)
    http.HandleFunc("/fibonacci", fibonacciHandler)
    fmt.Println("Go API running on port 5002")
    http.ListenAndServe(":5002", nil)
}
