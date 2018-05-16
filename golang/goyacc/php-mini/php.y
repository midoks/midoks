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
    "math"
    "os"
)

%}

%union{
    value float64
}


%token END         "end of file"
%token T_EXIT      "exit (T_EXIT)"
%token T_COMMENT   "comment (T_COMMENT)"
%token NUM

%left   '-' '+'
%left   '*' '/'
%left   NEG     /* negation--unary minus */
%right  '^'     /* exponentiation */

%type   <value> NUM, exp

%% /* Rules */

start:
    top_statement_list
;

top_statement_list:    /* empty */
        | top_statement_list line
;

line:     '\n'
        | exp '\n'  { fmt.Printf("\t%.10g\n", $1) }
;

exp:      NUM                { $$ = $1          }
        | exp '+' exp        { $$ = $1 + $3     }
        | exp '-' exp        { $$ = $1 - $3     }
        | exp '*' exp        { $$ = $1 * $3     }
        | exp '/' exp        { $$ = $1 / $3     }
        | '-' exp  %prec NEG { $$ = -$2         }
        | exp '^' exp        { $$ = math.Pow($1, $3) }
        | '(' exp ')'        { $$ = $2;         }
;
%%

func main() {
    //t := newLexer(bufio.NewReader(os.Stdin))
    //fmt.Printf("p:%s\n", t)
    os.Exit(yyParse(newLexer(bufio.NewReader(os.Stdin))))
}
