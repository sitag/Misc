# distutils: language = c++
# distutils: sources = vecInitialize.cpp

import cython
import numpy as np
cimport numpy as np
from libc.stdio cimport printf

# benchmark cython implementation against raw cpp
cdef extern from "vecInitialize.h":
    double fillSliceCpp(double* data, long* starts, long* ends, double* values, int n, int dim, int* errcode)

def fillslice(
    np.ndarray[double, ndim=1, mode="c"] data not None,
    np.ndarray[long, ndim=1, mode="c"] starts not None,
    np.ndarray[long, ndim=1, mode="c"] ends not None,
    np.ndarray[double, ndim=1, mode="c"] values not None):
    cdef int n = len(starts)
    assert n == len(ends)
    assert n == len(values)
    assert n > 0
    cdef int i
    cdef double integral = 0
    for i in range(n):
        data[starts[i] : ends[i]] += values[i]
        integral += values[i] *(<double>(ends[i] - starts[i]))
    return integral

cdef void __binslice(
    np.ndarray[double, ndim=1, mode="c"] data,
    np.ndarray[double, ndim=1, mode="c"] binned,
    int binsize):
    cdef int nBinned = len(binned)
    for i in range(nBinned):
        binned[i] = np.sum(data[i*binsize:(i+1)*binsize])

def binslice(np.ndarray[double, ndim=1, mode="c"] data not None, int binsize):
    cdef int n = len(data)
    cdef int nBinned = n/binsize
    if n > nBinned * binsize:
        nBinned = nBinned + 1
    cdef np.ndarray[double, ndim=1, mode="c"] binned
    binned = np.zeros(nBinned, dtype='double')
    __binslice(data, binned, binsize)  
    return binned

def fillslice_cpp(
    np.ndarray[double, ndim=1, mode="c"] data not None,
    np.ndarray[long, ndim=1, mode="c"] starts not None,
    np.ndarray[long, ndim=1, mode="c"] ends not None,
    np.ndarray[double, ndim=1, mode="c"] values not None):
    cdef int n = len(starts)
    assert n == len(ends)
    assert n == len(values)
    cdef int errcode = 1
    cdef int dim = len(data)
    cdef double integral = fillSliceCpp(&data[0], &starts[0], &ends[0], &values[0], n, dim, &errcode)
    assert errcode == 0 
    return integral

def fillslice_cpp_cython(starts, ends, values):
    data = np.zeros(20, dtype='float64')
    data_cpp = np.zeros(20, dtype='float64')
    n = len(starts)
    integral_cpp = fillslice_cpp(data_cpp, starts, ends, values)
    integral = fillslice(data, starts, ends, values)
    return {'cython' : { 'data'  : data, 'integral' :  integral}, 'cpp' : { 'data'  : data_cpp, 'integral' :  integral_cpp} }
    
 
