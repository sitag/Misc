import requests
from bs4 import BeautifulSoup

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
import sys
for arg in sys.argv[1:]:
	r = TitleUrl(arg)
	print "'{0}'\t{1}".format(r.title(), r.url)
