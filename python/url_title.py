#!/bin/env python
import sys

try:
	import requests
	from bs4 import BeautifulSoup
except:
	print 'need requests+bs4'
	sys.exit(1)
	
class TitleUrl:
	def eval(self):
		if not self.evaluated:
			self.evaluated = True
			self.requested = requests.get(self.url)
			self.soup = BeautifulSoup(self.requested.text, 'html.parser')
	def __init__(self, url):
		self.evaluated = None
		self.soup = None
		self.statusOk = True
		self.url = url
	def title(self):
		self.eval()
		return self.soup.title.text

def main(args):
	for arg in args:
		r = TitleUrl(arg)
		print "'{0}'\t{1}".format(r.title(), r.url)

if __name__ == '__main__':
	main(sys.argv[1:])
