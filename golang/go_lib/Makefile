all: main.go
	go build $<

run: all
	./main

clean:
	rm -f lib.so main

lib:
	go build -v -x -buildmode=c-shared -o lib.so