#include <stdio.h>
#include <stdlib.h>

typedef unsigned char	uchar;
typedef unsigned short uint16;
typedef unsigned int uint32;

void print_bin(int n);

static inline uint32 uint4korr(const uchar *A)
{

  printf("a[order uint4korr A:%u\n", (uint32)(*A));
  print_bin((uint32)(*A));
  printf("\n");

  printf("a[order uint4korr A[0] *]:%u\n",  A[0]);
  print_bin((uint32)(A[0]));
  printf("\n");


  printf("a[order uint4korr A[1] *]:%u\n", A[1]);
  print_bin((uint32)(A[1]));
  printf("\n");

  printf("a[order uint4korr A[2] *]:%u\n", A[2]);
  print_bin((uint32)(A[2]));
  printf("\n");


  printf("a[order uint4korr A[3] *]:%u\n", A[3]);
  print_bin((uint32)(A[3]));
  printf("\n");

  printf("a[order uint4korr A[0]]:%u\n", ((uint32) A[0]));
  printf("a[order uint4korr A[1]]:%u\n", ((uint32) A[1]));
  printf("a[order uint4korr A[2]]:%u\n", ((uint32) A[2]));
  printf("a[order uint4korr A[3]]:%u\n", ((uint32) A[3]));
  return
    (uint32) (((uint32) (A[0])) +
              (((uint32) (A[1])) << 8) +
              (((uint32) (A[2])) << 16) +
              (((uint32) (A[3])) << 24))
    ;
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
	printf("%s\n", "test uint4korr");

  uchar *p = NULL;
  p = malloc(sizeof(uchar)*1);
  *p = 'a';

	uchar *a = p;

  printf("a point len:%ld\n", sizeof(&a));

	printf("p:%s\n", p);
	print_bin((uint32)p);

	printf("\n");
	printf("a:%u\n", *a);
	print_bin((uint32)(*a));
	printf("\n");


	uint32 b = uint4korr(a);
	printf("a[order uint4korr b1]:%u\n", b);
	print_bin((uint32)b);
	printf("\n");

  printf("---------- seg -----------\n");

  printf("\n");

  print_bin((uint32)(a-1));
  printf("\n");

  uint32 b2 = uint4korr(a-1);
  printf("a[order uint4korr b2]:%u\n", b2);
  print_bin((uint32)b2);
  printf("\n");

  free(p);
}