package main

import (
	"net/http"
)

func main() {
	http.Handle("/", http.FileServer(http.Dir("/opt/dist")))
	http.ListenAndServe(":8080", nil)
}
