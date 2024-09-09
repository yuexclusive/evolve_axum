target=x86_64-unknown-linux-musl
name=evolve_axum

.PHONY: watch
watch:
	cargo watch -x "run"

.PHONY: clean
clean:
	docker images -f "dangling=true" | grep -v kindest | awk 'NR!=1{print $$3}' | xargs docker rmi

.PHONY: build
build:
	cargo zigbuild --release --target=${target}

.PHONY: image
image: build
	docker build --no-cache -f Dockerfile --platform=linux/amd64 --build-arg target=${target} --build-arg name=${name} -t yuexclusive/${name}:latest .
	make clean

MODULE_NAME:=evolve_axum
LNGUAGE:="rust"
.PHONY: openapi
openapi:
	openapi-generator generate --skip-validate-spec -i http://localhost:3000/api-docs/openapi.json -g ${LNGUAGE} --package-name ${MODULE_NAME}_cli -o ./${MODULE_NAME}_cli