%{
// Copyright (c) 2011 CZ.NIC z.s.p.o. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

// blame: jnml, labs.nic.cz

// +build ignore

package main

import (
    "bufio"
    "fmt"
    "os"
)

var (
    src      = bufio.NewReader(os.Stdin)
    buf      []byte
    current  byte
)

func getc() byte {
    if current != 0 {
        buf = append(buf, current)
    }
    current = 0
    if b, err := src.ReadByte(); err == nil {
        current = b
    }
    return current
}

func main() { // This left brace is closed by *1
    c := getc() // init
%}

%yyc c
%yyn c = getc()


LNUM	[0-9]+
DNUM	([0-9]*"."[0-9]+)|([0-9]+"."[0-9]*)
EXPONENT_DNUM	(({LNUM}|{DNUM})[eE][+-]?{LNUM})
HNUM	"0x"[0-9a-fA-F]+
BNUM	"0b"[01]+
LABEL	[a-zA-Z_\x80-\xff][a-zA-Z0-9_\x80-\xff]*
WHITESPACE [ \n\r\t]+
TABS_AND_SPACES [ \t]*
TOKENS [;:,.\[\]()|^&+-/*=%!~$<>?@]
ANY_CHAR [^]
NEWLINE ("\r"|"\n"|"\r\n")

%%
    buf = buf[:0]   // The code before the first rule executed before every scan cycle (rule #0 / state 0 action)

[ \t\n\r]+

{LNUM}              	fmt.Printf("LNUM %q\n", buf)

{DNUM}     				fmt.Printf("DNUM %q\n", buf)

{TOKENS}     			fmt.Printf("TOKENS %q\n", buf)

\<\?php  				fmt.Printf("string %q\n", buf)
\?>  					fmt.Printf("string %q\n", buf)


\0                  return

.                   fmt.Printf("%q\n", buf)

%%
    // The golex generated scanner enters top of the user code section when
    // lexem recongition fails. In this example it should never happen.
    panic("scanner internal error")

} // *1 this right brace
