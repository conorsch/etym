etym
=========

Simple Python script to query [EtymOnline] from the command line. 
Use `--random` if you have trouble choosing. Query from your terminal, 
or set as a motd.

Usage
-----
```
$ etym viking
Viking (n.)
Scandinavian pirate, 1807, vikingr; modern spelling attested from 1840. The
word is a historical revival; it was not used in Middle English, but it was
revived from Old Norse vikingr "freebooter, sea-rover, pirate, viking,"
which usually is explained as meaning properly "one who came from the
fjords," from vik "creek, inlet, small bay" (cf. Old English wic, Middle
High German wich "bay," and second element in Reykjavik). But Old English
wicing and Old Frisian wizing are almost 300 years older, and probably
derive from wic "village, camp" (temporary camps were a feature of the
Viking raids), related to Latin vicus "village, habitation" (see villa).
None None The connection between the Norse and Old English words is still
much debated. The period of Viking activity was roughly 8c. to 11c. In the
Anglo-Saxon Chronicle, the raiding armies generally were referred to as þa
Deniscan "the Danes," while those who settled in England were identified by
their place of settlement. Old Norse viking (n.) meant "freebooting voyage,
piracy;" one would "go on a viking."
```

Consider placing a call to this script with the `--random` option in your 
`~/.bash_logout` file if you hop between machines frequently, e.g.:

```sh
# ~/.bash_logout (or ~/.bash_login, if you prefer)
etym --random
```
The script will block for a moment, but the roundtrip is negible: 
```
$ time etym --random
cudgel (v.)
"to beat with a cudgel," 1590s, from cudgel (n.). Related: Cudgeled; cudgeling.

real    0m0.344s
user    0m0.137s
sys     0m0.032s
```
The above examples assume you've added the script to your `$PATH`. 

Installation
--------------

```
git clone https://github.com/ronocdh/etym
cd etym
pip install -r requirements.txt
```

Requirements
------------

* [requests]: Excellent HTTP library for humans.
* [BeautifulSoup]: Don't parse HTML without it.
* [blessings]: For text highlighting in terminal output.
* [docopt]: Command-line args that make you smile.
* List of words at `/usr/share/dict/words`
* An active internet connection.

License
----

MIT

[EtymOnline]:http://www.etymonline.com/
[requests]:http://docs.python-requests.org/en/latest/
[BeautifulSoup]:http://www.crummy.com/software/BeautifulSoup/
[blessings]:https://pypi.python.org/pypi/blessings/
[docopt]:http://docopt.org/

