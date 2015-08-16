# coding: utf-8
from BeautifulSoup import BeautifulSoup
import requests
query = 'viking'
r = requests.get("http://www.etymonline.com/index.php?search=" + query)
r.content
soup = BeautifulSoup(r.content, convertEntities=BeautifulSoup.HTML_ENTITIES)
soup
soup.dt.a.text
soup.dd
query = 'whale'
r = requests.get("http://www.etymonline.com/index.php?search=" + query)
soup = BeautifulSoup(r.content, convertEntities=BeautifulSoup.HTML_ENTITIES)
soup.dd
soup.dt.a.text
soup.dd
soup.dt
soup.dd
soup.dt
soup.dt()
soup.dd()
r.content
soup
soup.findAll('dd', { 'class': 'hightlight' } )
soup
soup.dd
[ x.a for x in soup.findAll('dt', { 'class': 'highlight' } ) ]
hits = [ x.a.text for x in soup.findAll('dt', { 'class': 'highlight' } ) ]
etyms = [ x.text for x in soup.findAll('dd', { 'class': 'highlight' } ) ]
results = zip(hits, etyms)
print(results)
