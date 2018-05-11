all: main.go
	go build $<

run: all
	./main

clean:
	rm -f lib.so main .DS_Store

so:
	go build -v -x -buildmode=c-shared -o lib.so

lib:
	go install -buildmode=shared -linkshared lib

exe:
	go build -buildmode=exe