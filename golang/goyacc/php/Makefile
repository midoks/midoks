all: lex.go yacc.go
	gofmt -l -s -w *.go
	go build $^

run: all
	./lex

lex.go: lex.l
	golex -o $@ $<

yacc.go: yacc.y
	goyacc -o $@ $<


clean:
	rm -f lex lex.go lex.yy.go yacc.go y.output *~

test:
	cat i.php | ./lex