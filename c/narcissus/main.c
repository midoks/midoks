#include <stdio.h>

//输出水仙花数
// a*a*a + b*b*b + c*c*c == i

int main(int argc, char const *argv[])
{
	int i,a,b,c;

	for (int i = 10; i < 1000000; ++i)
	{
		a=i/100;
		b=i%100/10;
		c=i%10;

		if ( a*a*a + b*b*b + c*c*c == i){
			printf("%d\n", i);
		}
	}

	//发出声音
	printf("\a");


	return 0;
}