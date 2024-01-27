package main

import (
	"bytes"
	"embed"
	"encoding/base64"
	"encoding/json"
	"image/png"
	"log"
	"net/http"
	"os"

	"github.com/ungerik/go-cairo"
)

//go:embed resources/*
var res embed.FS

type RequestBody struct {
	PfpB64    string `json:"pfp_b64"`
	RightRect Rect   `json:"right_rect"`
	LeftRect  Rect   `json:"left_rect"`
}

type Rect struct {
	X      float64 `json:"x"`
	Y      float64 `json:"y"`
	Width  float64 `json:"width"`
	Height float64 `json:"height"`
}

type ResponeBody struct {
	HtmxPfp string `json:"htmx_pfp"`
}

// func appendLaser(laserFileName string, surface *cairo.Surface) *cairo.Surface {
//
// }

func main() {
	http.Handle("/", http.FileServer(http.Dir("/opt/dist")))
	http.HandleFunc("/api/generate-htmx-pfp", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type")
		w.Header().Set("Access-Control-Allow-Methods", "GET,POST,PUT,PATCH,DELETE,OPTIONS")
		if r.Method == http.MethodOptions {
			return
		}
		var reqBody RequestBody
		err := json.NewDecoder(r.Body).Decode(&reqBody)
		if err != nil {
			w.WriteHeader(http.StatusBadRequest)
			return
		}

		imgDec, err := base64.StdEncoding.DecodeString(reqBody.PfpB64)
		if err != nil {
			log.Println(err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		img, err := png.Decode(bytes.NewReader(imgDec))
		if err != nil {
			log.Println(err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		workingImage := cairo.NewSurfaceFromImage(img)
		defer cleanSurface(workingImage)

		// right laser paint
		rightLaserFile, _ := os.Open("./resources/laser-right.png")
		rightLaserImage, _ := png.Decode(rightLaserFile)
		rightLaserSurface := cairo.NewSurfaceFromImage(rightLaserImage)

		defer cleanSurface(rightLaserSurface)

		rightLaserWidth, rightLaserHeight := float64(rightLaserSurface.GetWidth()), float64(rightLaserSurface.GetHeight())
		workingImage.Scale(reqBody.RightRect.Width/rightLaserWidth, reqBody.RightRect.Height/rightLaserHeight)

		workingImage.SetSourceSurface(rightLaserSurface, reqBody.RightRect.X, reqBody.RightRect.Y)
		workingImage.Paint()

		// left laser paint
		leftLaserFile, _ := os.Open("./resources/laser-left.png")
		leftLaserImage, _ := png.Decode(leftLaserFile)
		leftLaserSurface := cairo.NewSurfaceFromImage(leftLaserImage)

		defer cleanSurface(leftLaserSurface)

		leftLaserWidth, leftLaserHeight := float64(leftLaserSurface.GetWidth()), float64(leftLaserSurface.GetHeight())
		workingImage.Scale(reqBody.LeftRect.Width/leftLaserWidth, reqBody.LeftRect.Height/leftLaserHeight)

		workingImage.SetSourceSurface(rightLaserSurface, reqBody.LeftRect.X, reqBody.LeftRect.Y)
		workingImage.Paint()

		generatedImgBytes, status := workingImage.WriteToPNGStream()
		if status != cairo.STATUS_SUCCESS {
			log.Println(status.String())
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		pfpB64 := base64.StdEncoding.EncodeToString(generatedImgBytes)

		_ = json.NewEncoder(w).Encode(ResponeBody{
			HtmxPfp: pfpB64,
		})
	})

	http.ListenAndServe(":8080", nil)
}

func cleanSurface(surface *cairo.Surface) {
	surface.Finish()
	// surface.Destroy()
	surface.Flush()
}
