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

exit                    fmt.Printf("TOKEN:%q\n", buf)
die                     fmt.Printf("TOKEN:%q\n", buf)
function                fmt.Printf("TOKEN:%q\n", buf)
const                   fmt.Printf("TOKEN:%q\n", buf)
return                  fmt.Printf("TOKEN:%q\n", buf)
yield{WHITESPACE}from[\^a-zA-Z0-9_\x80-\xff] 
                        fmt.Printf("TOKEN:%q\n", buf)
yield                   fmt.Printf("TOKEN:%q\n", buf)
try                     fmt.Printf("TOKEN:%q\n", buf)
catch                   fmt.Printf("TOKEN:%q\n", buf)
finally                 fmt.Printf("TOKEN:%q\n", buf)
throw                   fmt.Printf("TOKEN:%q\n", buf)
if                      fmt.Printf("TOKEN:%q\n", buf)
elseif                  fmt.Printf("TOKEN:%q\n", buf)
endif                   fmt.Printf("TOKEN:%q\n", buf)
else                    fmt.Printf("TOKEN:%q\n", buf)
while                   fmt.Printf("TOKEN:%q\n", buf)
endwhile                fmt.Printf("TOKEN:%q\n", buf)
do                      fmt.Printf("TOKEN:%q\n", buf)
for                     fmt.Printf("TOKEN:%q\n", buf)
endfor                  fmt.Printf("TOKEN:%q\n", buf)
foreach                 fmt.Printf("TOKEN:%q\n", buf)
endforeach              fmt.Printf("TOKEN:%q\n", buf)
declare                 fmt.Printf("TOKEN:%q\n", buf)
enddeclare              fmt.Printf("TOKEN:%q\n", buf)
instanceof              fmt.Printf("TOKEN:%q\n", buf)
as                      fmt.Printf("TOKEN:%q\n", buf)
switch                  fmt.Printf("TOKEN:%q\n", buf)
endswitch               fmt.Printf("TOKEN:%q\n", buf)
case                    fmt.Printf("TOKEN:%q\n", buf)
default                 fmt.Printf("TOKEN:%q\n", buf)
break                   fmt.Printf("TOKEN:%q\n", buf)
continue                fmt.Printf("TOKEN:%q\n", buf)
goto                    fmt.Printf("TOKEN:%q\n", buf)
echo                    fmt.Printf("TOKEN:%q\n", buf)
print                   fmt.Printf("TOKEN:%q\n", buf)
class                   fmt.Printf("TOKEN:%q\n", buf)
interface               fmt.Printf("TOKEN:%q\n", buf)
trait                   fmt.Printf("TOKEN:%q\n", buf)
extends                 fmt.Printf("TOKEN:%q\n", buf)
implements              fmt.Printf("TOKEN:%q\n", buf)

new                     fmt.Printf("TOKEN:%q\n", buf)
clone                   fmt.Printf("TOKEN:%q\n", buf)
var                     fmt.Printf("TOKEN:%q\n", buf)

eval                    fmt.Printf("TOKEN:%q\n", buf)
include                 fmt.Printf("TOKEN:%q\n", buf)
include_once            fmt.Printf("TOKEN:%q\n", buf)
require                 fmt.Printf("TOKEN:%q\n", buf)
require_once            fmt.Printf("TOKEN:%q\n", buf)

namespace               fmt.Printf("TOKEN:%q\n", buf)
use                     fmt.Printf("TOKEN:%q\n", buf)

insteadof               fmt.Printf("TOKEN:%q\n", buf)
global                  fmt.Printf("TOKEN:%q\n", buf)
isset                   fmt.Printf("TOKEN:%q\n", buf)
empty                   fmt.Printf("TOKEN:%q\n", buf)
__halt_compiler         fmt.Printf("TOKEN:%q\n", buf)
static                  fmt.Printf("TOKEN:%q\n", buf)
abstract                fmt.Printf("TOKEN:%q\n", buf)
final                   fmt.Printf("TOKEN:%q\n", buf)
private                 fmt.Printf("TOKEN:%q\n", buf)
protected               fmt.Printf("TOKEN:%q\n", buf)
public                  fmt.Printf("TOKEN:%q\n", buf)
unset                   fmt.Printf("TOKEN:%q\n", buf)
list                    fmt.Printf("TOKEN:%q\n", buf)
array                   fmt.Printf("TOKEN:%q\n", buf)
callable                fmt.Printf("TOKEN:%q\n", buf)


\+\+                    fmt.Printf("TOKEN:%q\n", buf)
\-\-                    fmt.Printf("TOKEN:%q\n", buf)
===                     fmt.Printf("TOKEN:%q\n", buf)
!==                     fmt.Printf("TOKEN:%q\n", buf)
==                      fmt.Printf("TOKEN:%q\n", buf)
(!=|<>)                 fmt.Printf("TOKEN:%q\n", buf)
\<\=\>                  fmt.Printf("TOKEN:%q\n", buf)
\<\=                    fmt.Printf("TOKEN:%q\n", buf)
\>\=                    fmt.Printf("TOKEN:%q\n", buf)
\+\=                    fmt.Printf("TOKEN:%q\n", buf)
\-\=                    fmt.Printf("TOKEN:%q\n", buf)
\*\=                    fmt.Printf("TOKEN:%q\n", buf)
\*\*                    fmt.Printf("TOKEN:%q\n", buf)
\*\*\=                  fmt.Printf("TOKEN:%q\n", buf)
\/\=                    fmt.Printf("TOKEN:%q\n", buf)
\.\=                    fmt.Printf("TOKEN:%q\n", buf)
\%\=                    fmt.Printf("TOKEN:%q\n", buf)
\<\<\=                  fmt.Printf("TOKEN:%q\n", buf)
\>\>\=                  fmt.Printf("TOKEN:%q\n", buf)
\&\=                    fmt.Printf("TOKEN:%q\n", buf)
\|\=                    fmt.Printf("TOKEN:%q\n", buf)
\^\=                    fmt.Printf("TOKEN:%q\n", buf)
\|\|                    fmt.Printf("TOKEN:%q\n", buf)

\&\&                    fmt.Printf("TOKEN:%q\n", buf)
OR                      fmt.Printf("TOKEN:%q\n", buf)
AND                     fmt.Printf("TOKEN:%q\n", buf)
XOR                     fmt.Printf("TOKEN:%q\n", buf)
\<\<                    fmt.Printf("TOKEN:%q\n", buf)
\>\>                    fmt.Printf("TOKEN:%q\n", buf)
{TOKENS}                fmt.Printf("TOKEN:%q\n", buf)

{                       fmt.Printf("TOKEN:%q\n", buf)
\${                     fmt.Printf("TOKEN:%q\n", buf)
}                       fmt.Printf("TOKEN:%q\n", buf)
{LABEL}[[}]             fmt.Printf("TOKEN-LABEL2:%q\n", buf)



{BNUM}                  fmt.Printf("TOKEN-BNUM:%q\n", buf)
{LNUM}                  fmt.Printf("TOKEN-LNUM:%q\n", buf)
{HNUM}                  fmt.Printf("TOKEN-HNUM:%q\n", buf)
{DNUM}|{EXPONENT_DNUM}  fmt.Printf("TOKEN-HNUM:%q\n", buf)

__CLASS__               fmt.Printf("TOKEN:%q\n", buf)
__TRAIT__               fmt.Printf("TOKEN:%q\n", buf)
__FUNCTION__            fmt.Printf("TOKEN:%q\n", buf)
__METHOD__              fmt.Printf("TOKEN:%q\n", buf)
__LINE__                fmt.Printf("TOKEN:%q\n", buf)
__FILE__                fmt.Printf("TOKEN:%q\n", buf)
__DIR__                 fmt.Printf("TOKEN:%q\n", buf)
__NAMESPACE__           fmt.Printf("TOKEN:%q\n", buf)




->                      fmt.Printf("TOKEN:%q\n", buf)
=>                      fmt.Printf("TOKEN:%q\n", buf)

\${LABEL}\-\>[a-zA-Z_\x80-\xff]      
                        fmt.Printf("TOKEN:%q\n", buf)
\${LABEL}\[             fmt.Printf("TOKEN:%q\n", buf)
\${LABEL}               fmt.Printf("TOKEN:%q\n", buf)
{LABEL}                 fmt.Printf("TOKEN:%q\n", buf)

{TOKENS}|[{}"`]         fmt.Printf("TOKEN:%q\n", buf)

::                      fmt.Printf("TOKEN:%q\n", buf)
\\                      fmt.Printf("TOKEN:%q\n", buf)
\.\.\.                  fmt.Printf("TOKEN:%q\n", buf)
\?\?                    fmt.Printf("TOKEN:%q\n", buf)

({TABS_AND_SPACES}(int|integer){TABS_AND_SPACES})
                        fmt.Printf("TOKEN:%q\n", buf)
({TABS_AND_SPACES}(real|double|float){TABS_AND_SPACES})
                        fmt.Printf("TOKEN:%q\n", buf)
({TABS_AND_SPACES}(string|binary){TABS_AND_SPACES})
                        fmt.Printf("TOKEN:%q\n", buf)
({TABS_AND_SPACES}(array){TABS_AND_SPACES})
                        fmt.Printf("TOKEN:%q\n", buf)
({TABS_AND_SPACES}(object){TABS_AND_SPACES})
                        fmt.Printf("TOKEN:%q\n", buf)
({TABS_AND_SPACES}(bool|boolean){TABS_AND_SPACES})
                        fmt.Printf("TOKEN:%q\n", buf)
({TABS_AND_SPACES}(unset){TABS_AND_SPACES})
                        fmt.Printf("TOKEN:%q\n", buf)



\<\?\=                  fmt.Printf("TOKEN:%q\n", buf)
\<\?php  				fmt.Printf("TOKEN:%q\n", buf)
\?>  					fmt.Printf("TOKEN:%q\n", buf)

[ \n\r\t\\\'\#]           fmt.Printf("TOKEN:%q\n", buf)

.                       fmt.Printf("char:%q\n", buf)
\0                      
                        fmt.Printf("END\n")
                        return

%%
  
    // {WHITESPACE}+           fmt.Printf("TOKEN:%q\n", buf)
    // {ANY_CHAR}              fmt.Printf("TOKEN:%q\n", buf)
    // .                       fmt.Printf("char:%q\n", buf)
    // \0                      return
    // [0]|([1-9][0-9]*)       fmt.Printf("TOKEN-SNUM:%q\n", buf)
    // {LNUM}|{HNUM}|{BNUM}    fmt.Printf("TOKEN-CNUM:%q\n", buf)
    // ]      fmt.Printf("TOKEN:%q\n", buf)


    // The golex generated scanner enters top of the user code section when
    // lexem recongition fails. In this example it should never happen.
    panic("scanner internal error")

} // *1 this right brace
