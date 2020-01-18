#include <stdio.h>
#include <stdlib.h>

typedef unsigned char uchar;
typedef unsigned short uint16;
typedef unsigned int uint32;
typedef unsigned int uint;

void print_bin(int n);

static inline void int2store(uchar *T, uint16 A)
{
  uint def_temp= A ;
  printf("p[int2store b2]:%hu[i2s]\n", A);
  print_bin((uint16)(A));
  printf("\n");

  *(T)=   (uchar)(def_temp);
  printf("p[int2store b2]:%hhu[i2s]\n", *(T));
  print_bin((uint16)(*T));
  printf("\n");

  *(T+1)= (uchar)(def_temp >> 8);
  printf("p[int2store b2]:%hhu[i2s]\n", *(T+1));
  print_bin((uint16)(*T+1));
  printf("\n");
}

void print_bin(int n)
{
    int l = sizeof(n)*8;//总位数。
    int i;
    if(i == 0)
    {
         printf("0");
         return;
     }
    for(i = l-1; i >= 0; i --)//略去高位0.
    {
        if(n&(1<<i)) break;
    }
 
    for(;i>=0; i --)
        printf("%d", (n&(1<<i)) != 0);
}


int main(){
  printf("%s\n", "test int2store!");

  uchar *p = NULL;
  p = malloc(sizeof(uchar)*4);

  printf("a point len:%ld\n", sizeof(p));

  printf("p-ld:%d\n", 0xFFF);
  printf("p-ld:%d\n", 0xFFF >> 8);

  uint16 b;
  int2store(p,0xFFF);
  printf("p[order int2store b1]:%hhu[s]\n", *p);
  print_bin((uint16)(*p));
  printf("\n");

  free(p);
  return 0;
}