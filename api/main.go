package main

import (
	"encoding/json"
	"fmt"
	"net/http"
)

type ResponeBody struct {
	HtmxPfp string `json:"htmx_pfp"`
}

func main() {
	http.Handle("/", http.FileServer(http.Dir("/opt/dist")))
	http.HandleFunc("/api/generate-htmx-pfp", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type")
		w.Header().Set("Access-Control-Allow-Methods", "GET,POST,PUT,PATCH,DELETE,OPTIONS")
		if r.Method == http.MethodOptions {
			return
		}
		req := map[string]any{}
		_ = json.NewDecoder(r.Body).Decode(&req)
		fmt.Println(req)
		_ = json.NewEncoder(w).Encode(ResponeBody{
			HtmxPfp: "AAAAAAA",
		})
	})

	http.ListenAndServe(":8080", nil)
}
