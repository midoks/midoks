
all: php.yacc.go php.lex.go
	gofmt -l -s -w *.go
	go build $^
	mv php.yacc php

run: all
	cat i.php | ./php

php.lex.go: php.l
	golex -o $@ $<

php.yacc.go: php.y
	goyacc -o $@ $<


clean:
	rm -f php php.yacc.go php.lex.go php.lex.go php.yacc.go php.yacc y.output *~

test:
	cat i.php | ./php

test7:
	cat i.php | /Applications/mdserver/bin/php/php72d/bin/php 


iphp:
	sh /Applications/mdserver/bin/reinstall/php72.3_log.sh