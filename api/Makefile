.PHONY: build

BINARY_NAME=hppm

build:
	go mod tidy && \
	go build -ldflags="-w -s" -o ${BINARY_NAME}


# install inotify-tools
dev:
	while true; do \
	  go build -o ${BINARY_NAME}; \
	  ./${BINARY_NAME} & \
	  PID=$$!; \
	  echo "PID=$$PID"; \
	  inotifywait -r -e modify ./**; \
	  kill $$PID; \
	done

clean:
	go clean
