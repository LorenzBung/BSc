#include <stdio.h>

#include "mythreads.h"

int balance = 0;
pthread_mutex_t mut;

void* worker(void* arg) {
    pthread_mutex_lock(&mut);
    balance++; // protected access
    pthread_mutex_unlock(&mut);
    return NULL;
}

int main(int argc, char *argv[]) {
    pthread_t p;
    Pthread_create(&p, NULL, worker, NULL);
    pthread_mutex_lock(&mut);
    balance++; // unprotected access
    pthread_mutex_unlock(&mut);
    Pthread_join(p, NULL);
    return 0;
}
