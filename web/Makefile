.PHONY: build

build:
	cd tailwindcss && \
	npx tailwindcss@3.4.1 -i ../index.css -o ../tailwind.css -m && \
	cd .. && \
	cargo install --path . && trunk build


# install inotify-tools
dev:
	cd tailwindcss && \
	npx tailwindcss@3.4.1 -i ../index.css -o ../tailwind.css --watch & \
	trunk serve

clean:
	cargo clean
