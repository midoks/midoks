#include <pthread.h>
#include <stdio.h>
#include <unistd.h>
  
pthread_mutex_t mutex;

void *print_msg(void *arg){  
    int i=0;  
    pthread_mutex_lock(&mutex);  
    for(i=0;i<15;i++){  
            printf("output : %d\n",i);  
            usleep(100);  
    }  
    pthread_mutex_unlock(&mutex);

    printf("----------\n");

    return (void *)0;
}  


int main(int argc,char** argv){  
    pthread_t id1;  
    pthread_t id2;  
    pthread_mutex_init(&mutex,NULL);  
    pthread_create(&id1,NULL,print_msg,NULL);  
    pthread_create(&id2,NULL,print_msg,NULL);  
    pthread_join(id1,NULL);  
    pthread_join(id2,NULL);  
    pthread_mutex_destroy(&mutex);  
    return 1;  
}  