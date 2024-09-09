
MODULE_NAME:=evolve_axum
LNGUAGE:="rust"
.PHONY: openapi
openapi:
	openapi-generator generate --skip-validate-spec -i http://localhost:3000/api-docs/openapi.json -g ${LNGUAGE} --package-name ${MODULE_NAME}_cli -o ./${MODULE_NAME}_cli