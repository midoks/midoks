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

test7:
	cat i.php | /Applications/mdserver/bin/php/php72d/bin/php 


iphp:
	sh /Applications/mdserver/bin/reinstall/php72_log.sh