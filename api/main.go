package main

import (
	"bytes"
	"embed"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"image/png"
	"log"
	"math"
	"net/http"
	"os"

	"github.com/ungerik/go-cairo"
)

//go:embed resources/*
var res embed.FS

const (
	OriginalImageWidth  = 365
	OriginalImageHeight = 365

	OriginalRightLaserXPadding = 43.33
	OriginalRightLaserYPadding = 46.94
	OriginalLeftLaserXPadding  = 41.17
	OriginalLeftLaserYPadding  = 44.62
)

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

// getCenterStartOfElement return the coordinate of the first point of the child element
// that will allow it to appear in the middle of the parent
// hmm you want the math, fine!
// we have p as the parent's length and c as the child's length soooo
// we want the coordinate(x or y) that will make the child appear in the middle
// ie (p-(p-c))/2 the middle of the difference of the the difference between child and parent
// ok some magical math properties will get us to p-c/2
// (p-(p-c))/2 = ((p-p)-(p-c))/2
// = (-(p-c))/2 = |(p-c)/2| SINCE IT'S A LENGTH BLYAT!!
func getCenterStartOfElement(childLength float64, parentLength float64) float64 {
	return math.Abs((parentLength - childLength) / 2.0)
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
		imgWidth, imgHeight := img.Bounds().Dx(), img.Bounds().Dy()
		_, _ = imgWidth, imgHeight
		workingImage := cairo.NewSurfaceFromImage(img)
		defer cleanSurface(workingImage)

		fmt.Printf("before right: %+v\n", reqBody.RightRect)
		fmt.Printf("before left: %+v\n", reqBody.LeftRect)

		/// add/remove padding from the lasers' coordinates
		// added padding to the lasers' coordinates
		//	var widthPadding, heightPadding float64

		//	fmt.Printf("orig { w: %d, h: %d }\nnew { w: %d, h: %d }\n", OriginalImageWidth, OriginalImageHeight, imgWidth, imgHeight)

		//	switch {
		//	case imgWidth < OriginalImageWidth:
		//		widthPadding = -getCenterStartOfElement(float64(imgWidth), OriginalImageWidth)

		//	case imgWidth > OriginalImageWidth:
		//		widthPadding = getCenterStartOfElement(float64(imgWidth), OriginalImageWidth)
		//	}

		//	switch {
		//	case imgHeight < OriginalImageHeight:
		//		heightPadding = -getCenterStartOfElement(float64(imgHeight), OriginalImageHeight) / 2.0

		//	case imgHeight > OriginalImageHeight:
		//		heightPadding = getCenterStartOfElement(float64(imgHeight), OriginalImageHeight) / 2.0
		//	}
		//	// widthPadding = -(float64(OriginalImageWidth - imgWidth)) / 2.
		//	// heightPadding = -(float64(OriginalImageHeight - imgHeight)) / 4.

		//	reqBody.RightRect.X += float64(widthPadding)
		//	reqBody.RightRect.Y += float64(heightPadding)
		//	reqBody.LeftRect.X += float64(widthPadding)
		//	reqBody.LeftRect.Y += float64(heightPadding)

		//	fmt.Printf("after right: %+v\n", reqBody.RightRect)
		//	fmt.Printf("after left: %+v\n", reqBody.LeftRect)
		/// fin

		// scaling shits
		reqBody.RightRect.Width *= float64(imgWidth) / float64(OriginalImageWidth)
		reqBody.RightRect.Height *= float64(imgHeight) / float64(OriginalImageHeight)
		reqBody.LeftRect.Width *= float64(imgWidth) / float64(OriginalImageWidth)
		reqBody.LeftRect.Height *= float64(imgHeight) / float64(OriginalImageHeight)

		reqBody.RightRect.X *= float64(imgWidth) / float64(OriginalImageWidth)
		reqBody.RightRect.Y *= float64(imgHeight) / float64(OriginalImageHeight)
		reqBody.LeftRect.X *= float64(imgWidth) / float64(OriginalImageWidth)
		reqBody.LeftRect.Y *= float64(imgHeight) / float64(OriginalImageHeight)

		// right laser paint
		rightLaserFile, _ := os.Open("./resources/laser-right.png")
		rightLaserImage, _ := png.Decode(rightLaserFile)
		rightLaserSurface := cairo.NewSurfaceFromImage(rightLaserImage)
		rightLaserContainerSurface := cairo.NewSurface(cairo.FORMAT_ARGB32, int(reqBody.RightRect.Width), int(reqBody.RightRect.Height))

		defer cleanSurface(rightLaserSurface)

		rightLaserWidth, rightLaserHeight := float64(rightLaserSurface.GetWidth()), float64(rightLaserSurface.GetHeight())
		rightLaserContainerSurface.Scale(reqBody.RightRect.Width/rightLaserWidth, reqBody.RightRect.Height/rightLaserHeight)

		rightLaserContainerSurface.SetSourceSurface(rightLaserSurface, 0, 0)
		rightLaserContainerSurface.Paint()

		// left laser paint
		leftLaserFile, _ := os.Open("./resources/laser-left.png")
		leftLaserImage, _ := png.Decode(leftLaserFile)
		leftLaserSurface := cairo.NewSurfaceFromImage(leftLaserImage)
		leftLaserContainerSurface := cairo.NewSurface(cairo.FORMAT_ARGB32, int(reqBody.LeftRect.Width), int(reqBody.LeftRect.Height))

		defer cleanSurface(leftLaserSurface)

		leftLaserWidth, leftLaserHeight := float64(leftLaserSurface.GetWidth()), float64(leftLaserSurface.GetHeight())
		leftLaserContainerSurface.Scale(reqBody.LeftRect.Width/leftLaserWidth, reqBody.LeftRect.Height/leftLaserHeight)

		leftLaserContainerSurface.SetSourceSurface(leftLaserSurface, 0, 0)
		leftLaserContainerSurface.Paint()

		workingImage.SetSourceSurface(rightLaserContainerSurface, reqBody.RightRect.X, reqBody.RightRect.Y)
		workingImage.Paint()

		workingImage.SetSourceSurface(leftLaserContainerSurface, reqBody.LeftRect.X, reqBody.LeftRect.Y)
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
