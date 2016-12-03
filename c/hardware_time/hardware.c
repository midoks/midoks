#include <stdio.h>
#include <sys/time.h>
#include <sys/resource.h>
#include <stdlib.h>
#include <unistd.h>

#if !defined(uint64)
typedef unsigned long long uint64;
#endif
#if !defined(uint32)
typedef unsigned int uint32;
#endif
#if !defined(uint8)
typedef unsigned char uint8;
#endif

/**
 *  * Get time stamp counter (TSC) value via 'rdtsc' instruction.
 *   *
 *    * @return 64 bit unsigned integer
 *     * @author cjiang
 *      */
static inline uint64 cycle_timer() {
	  uint32 __a,__d;
	    uint64 val;
		  asm volatile("rdtsc" : "=a" (__a), "=d" (__d));
		    (val) = ((uint64)__a) | (((uint64)__d)<<32);
			  return val;
}


int main(){
	uint64 s = cycle_timer();

	cycle_timer();

	uint64 e = cycle_timer();

	printf("time:%llu\n", e - s);
	return 0;
}
