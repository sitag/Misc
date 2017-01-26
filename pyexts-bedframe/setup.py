from distutils.core import setup
from distutils.cmd import Command
from Cython.Build import cythonize

class TestCommand(Command):
	user_options = []
	def initialize_options(self):
		pass
	def finalize_options(self):
		pass
	def run(self):
		import subprocess, sys
		raise SystemExit(subprocess.call([sys.executable, '-m', 'unittest', 'discover']))

setup(
 ext_modules = cythonize(
    "cexts.pyx", #cython source
    #sources=["vecInitialize.cpp"], # additional cpp source
    language="c++",# generate C++ code
), cmdclass={'test': TestCommand })


