
#include <stdio.h>
#include <stdarg.h>
#include <string.h>

int demo(const char *msg , ...)
{	
	int argno = 0;
	va_list ap;
	va_start(ap, msg);
	char *para;

	while (1){
		para = va_arg(ap, char*);
		
		if ( para == NULL ) {
			break;
		}

		if ( strcmp( para, "\0") == 0 ) {

			break;
		}

		printf("parameter #%d is: %s\n", argno, para);
		argno++;
	}
	va_end(ap);

	return 0;
}

int main(int argc, char const *argv[])
{
//mac have error,but linux no error
// parameter #0 is: 2
// parameter #1 is: s
// parameter #2 is: 3
// parameter #3 is: 3
// Segmentation fault: 11
	//demo("test","2","s","3");
	return 0;
}