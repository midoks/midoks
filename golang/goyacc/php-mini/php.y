/*

This file is a modified excerpt from the GNU Bison Manual examples originally found here:
http://www.gnu.org/software/bison/manual/html_node/Infix-Calc.html#Infix-Calc

The Copyright License for the GNU Bison Manual can be found in the "fdl-1.3" file.

*/

/* Infix notation calculator.  */

%{

// +build ignore

package main

import (
    "bufio"
    "fmt"
    //"math"
    "os"
)

%}

%union{
    value float64
}


%token END         "end of file"
%token T_EXIT      "exit (T_EXIT)"
%token T_COMMENT   "comment (T_COMMENT)"
%token T_LNUMBER   "integer number (T_LNUMBER)"
%token T_DNUMBER   "floating-point number (T_DNUMBER)"
%token T_STRING    "identifier (T_STRING)"

%left   '+' '-' '.'
%left   '*' '/' '%'


%type  <value>  T_DNUMBER expr

%% /* Rules */

start:
    top_statement_list
;



top_statement_list:
    expr
;

expr:   T_DNUMBER   { fmt.Printf("\t%.10g\n", $1 ); }
        | expr '+' expr  { fmt.Printf("\t%.10g:%.10g\n", $1 ,$3);    }
        | expr '-' expr  { fmt.Printf("\t%.10g:%.10g\n", $1 ,$3);    }
;



%%

func main() {
    //t := newLexer(bufio.NewReader(os.Stdin))
    //fmt.Printf("p:%p\n", t)
    os.Exit(yyParse(newLexer(bufio.NewReader(os.Stdin))))
}
