etymology
=========

Simple Python script to query [EtymOnline] from the command line. 
Right now it doesn't have any `--help` info, just looks up the word or phrase 
you specify on the command line. If you don't specify anything, it'll look up 
a random word from `/usr/share/dict/words` and display its etymology.

It fails gracefully if there's no network connection, thanks to [requests].

Usage
-----

```sh
$ ./etymology viking
Viking (n.)
Scandinavian pirate, 1807, vikingr; modern spelling attested from 1840. The word is a historical revival; it was not used in Middle English, but it was
revived from Old Norse vikingr "freebooter, sea-rover, pirate, viking," which usually is explained as meaning properly "one who came from the fjords,"
from vik "creek, inlet, small bay" (cf. Old English wic, Middle High German wich "bay," and second element in Reykjavik). But Old English wicing and
Old Frisian wizing are almost 300 years older, and probably derive from wic "village, camp" (temporary camps were a feature of the Viking raids),
related to Latin vicus "village, habitation" (see villa). None None The connection between the Norse and Old English words is still much debated. The
period of Viking activity was roughly 8c. to 11c. In the Anglo-Saxon Chronicle, the raiding armies generally were referred to as þa Deniscan "the
Danes," while those who settled in England were identified by their place of settlement. Old Norse viking (n.) meant "freebooting voyage, piracy;" one
would "go on a viking."
```

Installation
--------------

```sh
git clone https://github.com/ronocdh/etym
cd etym
pip install -r requirements.txt
```

Requirements
------------

* [requests]: Excellent HTTP library for humans.
* [BeautifulSoup]: Don't parse HTML without it.
* [nltk]: Used to lemmatize words pulled out of `/usr/share/dict/words`. Overkill, but fun to have around. Also provides access to a much larger English word corpus than the system dictionary, but that's not used right now.
* [blessings]: For text highlighting in terminal output.
* An active internet connection.


License
----

MIT

[EtymOnline]:http://www.etymonline.com/
[requests]:http://docs.python-requests.org/en/latest/
[BeautifulSoup]:http://www.crummy.com/software/BeautifulSoup/
[nltk]:http://www.nltk.org/
[blessings]:https://pypi.python.org/pypi/blessings/
