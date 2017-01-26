#include<iostream>
#include<string.h>
using namespace std;

extern "C" {

typedef struct Tuple {
    int a;
    int b;
} Tuple;


Tuple tupleNew(){
   Tuple test;
   test.a = 1;
   test.b = 2;
   return test;
}

char* strInterface(char* args[], int n){
    for(int i = 0 ; i< n; i++){
        cout << "cpp:" << args[i] << ":" <<  strlen(args[i]) << endl;
    }
    int k = strlen(args[0]);
    char* dest = new char[k+1];
    memcpy(dest, args[0], k);
    dest[k] = '!';
    cout << ".." << endl;
    return dest;
}

void dot(double* a, unsigned int size_a, double* b, unsigned int size_b){
    for (int i = 0; i < size_a; i++) {
      a[i] = a[i] * b[i];
    }
}

}


