

golex -o php.lex.go php.l
goyacc -o php.yacc.go php.y

gofmt -l -s -w php.yacc.go php.lex.go
go build php.yacc.go php.lex.go

ren php.yacc.exe php.exe


pause