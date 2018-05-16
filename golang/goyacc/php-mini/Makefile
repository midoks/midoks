all: php.lex.go php.yacc.go
	gofmt -l -s -w *.go
	go build $^
	mv php.lex php

run: all
	./php

php.lex.go: php.l
	golex -o $@ $<

php.yacc.go: php.y
	goyacc -o $@ $<


clean:
	rm -f php php.yacc.go php.lex.go php.lex.go php.yacc.go y.output *~

test:
	cat i.php | ./php