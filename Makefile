# Golang demo
.PHONY: watch test

main:
	go build -o main -compiler="gccgo" -gccgoflags="-g"

watch:
	gin -a 8080 -p 3000 -b main --all main.go \
	& browser-sync start -f './main' -p localhost:3000 --reload-delay 500

test:
	go test ./... -v
	go test ./... -race
