#include <stdio.h>

//9x9 乘法表

int main(int argc, char const *argv[])
{
	int i,j;

	printf("   ");
	for (i=1; i<=9; ++i)
	{
		printf("%3d", i);
	}
	printf("\n");


	for (i=1; i<=9; ++i)
	{
		printf("%3d", i);

		for (j=1; j<=9; ++j)
		{
			if (i>=j){
				printf("%3d", i*j);
			}
		}
		printf("\n");
	}
	return 0;
}