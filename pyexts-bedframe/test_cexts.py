import unittest
import cexts
import numpy as np

class TestBedFrame(unittest.TestCase):
	def test_fillslice(self):
		starts = np.array([0, 10])
		ends = np.array([5, 15])
		values = np.array([1.0,2.0])
		data = cexts.fillslice_cpp_cython(starts, ends, values)
		expected_integral = 15.0
		expected_data = np.array([1., 1., 1.,1.,1.,0.,0.,0.,0.,0.,2.,2.,2.,2.,2.,0.,0.,0.,0.,0.])
		self.assertEqual(data['cython']['integral'], data['cpp']['integral'])
		self.assertEqual(data['cython']['integral'], expected_integral)
		self.assertTrue(np.allclose(data['cython']['data'], data['cpp']['data']))
		self.assertTrue(np.allclose(data['cython']['data'], expected_data))
		print 'OK:test_fillslice'

def runtests():
	unittest.main()


	

if __name__ == '__main__':
	runtests()
	
    
