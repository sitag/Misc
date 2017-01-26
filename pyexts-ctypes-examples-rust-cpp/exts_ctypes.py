import ctypes
import numpy as np 

class NPTypes:
	@staticmethod
	def ptrOf(x):
		return x.ctypes.data_as(ctypes.POINTER(ctypes.c_double))

class PyRust:
	def __init__(self, so):
		self.lib = ctypes.cdll.LoadLibrary(so)

	def testsumlist(self):
		self.lib.test_parallel_list_sum.argtypes = (ctypes.POINTER(ctypes.c_int32), ctypes.c_size_t, ctypes.POINTER(ctypes.c_int32), ctypes.c_size_t)
		listA = list(range(5))
		listB = list(range(10))
		cListA = (ctypes.c_int32 * len(listA))(*listA)
		cListB = (ctypes.c_int32 * len(listB))(*listB)
		print sum(listA) + sum(listB),  self.lib.test_parallel_list_sum(cListA, len(cListA), cListB, len(cListB))
	
	def teststrings(self):
		testList = ['one', '=1', 'two', '=2']
		self.lib.test_cstr_interface.argtypes = (ctypes.POINTER(ctypes.c_char_p), ctypes.c_size_t)
		cList = (ctypes.c_char_p * len(testList))(*testList)
		returnString =  self.lib.test_cstr_interface(cList, len(testList))
		resultString = ctypes.c_char_p(returnString).value  
		self.lib.consume_cstr(returnString)
		return resultString

	def testjson(self):
		testJson = '{"hello" : "..."}'
		self.lib.test_cstr_interface.argtypes = (ctypes.c_char_p, )
		returnString =  self.lib.test_json_interface(testJson)
		resultString = ctypes.c_char_p(returnString).value  
		self.lib.consume_cstr(returnString)
		return resultString

	@staticmethod
	def main():
		rsso="./src-rust/target/release/librs.so"
		pyrust = PyRust(rsso)
		pyrust.testsumlist()
		print pyrust.teststrings()
		print "... " + pyrust.testjson()


class Tuple(ctypes.Structure):
	_fields_ = [("a",ctypes.c_int), ("b", ctypes.c_int)]

class PyCpp:
	def __init__(self, so):
		self.lib = ctypes.cdll.LoadLibrary(so)
		self.lib.strInterface.argtypes = (ctypes.POINTER(ctypes.c_char_p), ctypes.c_size_t)
		self.lib.strInterface.restype = ctypes.c_char_p
		self.lib.tupleNew.restype = Tuple
		self.lib.dot.argtypes = (ctypes.POINTER(ctypes.c_double), ctypes.c_uint, ctypes.POINTER(ctypes.c_double), ctypes.c_uint)

	def strInterface(self, strlist):
		# look at ctypes create_string_buffer 
		cList = (ctypes.c_char_p * len(strlist))(*strlist)
		returnString =  self.lib.strInterface(cList, len(strlist))
		return str(returnString)

	def npArray(self, numpyArray):
		ptr = NPTypes.ptrOf(numpyArray)
		size = numpyArray.shape[0]
		self.lib.dot(ptr, size, ptr, size)
		return numpyArray

	def tupleNew(self):
		return self.lib.tupleNew()

	@staticmethod
	def main():
		cppso="./src-cpp/lib/libcpp.so"
		pycpp = PyCpp(cppso)
		print pycpp.strInterface(["hello", "world"])
		print pycpp.npArray(np.array([2.0,3.0,4.0]))
		x = pycpp.tupleNew()
		print x.a, x.b


if __name__ == '__main__':
	PyCpp.main()
	PyRust.main()
	print '..ok..'



