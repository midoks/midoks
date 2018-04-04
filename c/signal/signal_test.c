#include <stdio.h>
#include <unistd.h>
#include <signal.h>

void handler()
{
	printf("hello handler\n");

}

int main(int argc, char const *argv[])
{
	
	signal(SIGALRM, handler);
	alarm(5);

	for (int i = 0; i < 7; ++i)
	{
		printf("sleep:%d\n", i);
		sleep(1);
	}

	return 0;
}