all: lex.go
	go build $<

run: all
	./lex

lex.go: lex.l
	golex -o $@ $<
	#echo `golex -l $< | gofmt > $@`

clean:
	rm -f lex lex.go lex.yy.go y.output *~