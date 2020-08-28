etym
=========

Single, statically-linked binary to query [EtymOnline] from the command line.
It formats the output nicely, using bold for keywords and italic for foreign words.

Installation
------------

First, make sure you have [Rust installed](https://rustup.rs/). Then:

```
cargo install --force etym
```

You can also build a deb package locally:

```
git clone https://github.com/conorsch/etym
cd etym
cargo deb
```

The package will be available at `target/debian/*.deb`.

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

Requirements
------------

* Assumes Linux, but should be easy enough to build for other targets
* Requires the [Rust toolchain](https://rustup.rs) to build.

License
----

MIT

[EtymOnline]:http://www.etymonline.com/
