#include "vecInitialize.h"
#include<iostream>

double fillSliceCpp(double* slice, long* starts, long* ends, double* values, int n, int dim, int* errcode){
	*errcode = 1;
	double integral = 0;
	for(int i = 0; i < n; i++){
		int i_start = *(starts+i);
		int i_end = *(ends+i);
		double v = *(values + i);
		if(i_end >= dim || i_start > i_end){
			return 0;
		}
		integral += ((double)(i_end - i_start))*v;
		double* iter = slice + i_start;
		for(int j = i_start; j < i_end; j++){
			*iter += v;
			iter++;
		}
	}
	*errcode = 0;
	return integral;
}
