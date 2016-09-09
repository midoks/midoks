#! /bin/sh

re2c -o re2c.cc re2c_01.re
g++ -o t re2c.cc

