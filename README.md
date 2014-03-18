etym
=========

Simple Python script to query [EtymOnline] from the command line. 
It formats the output nicely, using bold for keywords and italic for foreign words. 
Use `--random` if you have trouble choosing. Query from your terminal, 
or set as a motd. 

Usage
-----

<pre><code>$ etym viking
<b>Viking (n.)</b>
Scandinavian pirate, 1801, <i>vikingr</i>, in "The History of the Anglo-Saxons" 
by English historian Sharon H. Turner (1768-1847); he suggested the second element 
might be connected to <i>king</i>: The name by which the pirates were at first 
distinguished was Vikingr, which perhaps originally meant kings of the bays. 
It was in bays that they ambushed, to dart upon the passing voyager. But this 
later was dismissed as incorrect. The form <i>viking</i> is attested in 1820, 
in Jamieson's notes to "The Bruce." The word is a historians' revival; it was 
not used in Middle English, but it was reintroduced from Old Norse <i>vikingr</i> 
"freebooter, sea-rover, pirate, viking," which usually is explained as meaning 
properly "one who came from the fjords," from <i>vik</i> "creek, inlet, small bay" 
(cf. Old English <i>wic</i>, Middle High German <i>wich</i> "bay," and second 
 element in <i>Reykjavik</i>). But Old English <i>wicing</i> and Old Frisian <i>wizing</i> 
are almost 300 years older than the earliest attestation of the Old Norse word, 
and probably derive from <i>wic</i> "village, camp" (large temporary camps were 
a feature of the Viking raids), related to Latin <i>vicus</i> "village, habitation" (see villa). 
The connection between the Norse and Old English words is still much debated. 
The period of Viking activity was roughly 8c. to 11c. In the Anglo-Saxon Chronicle, 
the raiding armies generally were referred to as <i>þa Deniscan</i> "the Danes," while
those who settled in England were identified by their place of settlement. 
Old Norse <i>viking</i> (n.) meant "freebooting voyage, piracy;" one would
"go on a viking" ( <i>fara í viking</i>).
</code></pre>

Consider placing a call to this script with the `--random` option in your 
`~/.bash_logout` file if you hop between machines frequently, e.g.:

```sh
# ~/.bash_logout (or ~/.bash_login, if you prefer)
etym --random
```
The script will block for a moment, but the roundtrip is negible: 

<pre><code>$ time etym --random
<b>cudgel (v.)</b>
"to beat with a cudgel," 1590s, from cudgel (n.). Related: <i>Cudgeled</i>; <i>cudgeling</i>.

real    0m0.344s
user    0m0.137s
sys     0m0.032s
</code></pre>

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
* [blessings]: For rich text formatting in terminal output.
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

