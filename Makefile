# Golang demo

.PHONY: test

main:
	go build -o main -compiler="gccgo" -gccgoflags="-g"

test:
	go test ./...
