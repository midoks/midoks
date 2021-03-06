#include <stdio.h>
#include <stdlib.h>

typedef unsigned char	uchar;
typedef unsigned short uint16;

void print_bin(int n);

static inline uint16 uint2korr(const uchar *A)
{
  printf("a[order uint2korr A:%hu\n", (uint16)(*A));
  print_bin((int)(*A));
  printf("\n");

  printf("a[order uint2korr A[0] *]:%hhu\n",  A[0]);

  print_bin((int)(A[0]));
  printf("\n");


  printf("a[order uint2korr A[1] *]:%hhu\n", A[1]);
  print_bin((int)(A[1]));
  printf("\n");

  printf("a[order uint2korr A[0]]:%hu\n", ((uint16) A[0]));
  printf("a[order uint2korr A[1]]:%hu\n", ((uint16) A[1]));
  return
    (uint16) (((uint16) (A[0])) +
              ((uint16) (A[1]) << 8))
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
// static inline void print_itoa(int a){
// 	char str[30];
//     itoa(a,str,2);//2即是代表转换为2进制
//     printf("转换成二进制数是: %s\n\n",str);

// }

int main(){
	printf("%s\n", "test");

  uchar *a1 = NULL;
  a1 = malloc(sizeof(uchar)*1);
  *a1 = 0x1;

	uchar *a = a1;



	printf("a1:%s\n", a1);
	printf("a1[int]:%s\n", a1);
	print_bin((int)a1);

	printf("\n");
	printf("a:%s\n", a);
	print_bin((int)(*a));
	printf("\n");


	uint16 b = uint2korr(a);
	printf("a[order uint2korr]:%hu\n", b);
	print_bin((int)b);
	printf("\n");

  free(a1);
}