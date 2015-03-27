def beautify(soup):
    """Parse BeautifulSoup HTML and return prettified string."""
    # BeautifulSoup strips out whitespace around in-line markup tags, see 
    # http://stackoverflow.com/a/16767636 for explanation of solution used below.
    beautifiedText = str()
    term = Terminal()
    for i in soup:
        if i.string:
            if re.match(r'<span class="foreign">', str(i), re.UNICODE):
                i.string = re.sub(r'<span class="foreign">(.+)</span>', r'{t.standout}\1{t.normal}'.format(t=term), str(i))
            beautifiedText += ' ' + i.string

    # Clean up 
    beautifiedText = re.sub('^\s+', '', beautifiedText)
    beautifiedText = re.sub('\s{2,}', ' ', beautifiedText)
    beautifiedText = re.sub('\s+([,)\].;:])', '\g<1>', beautifiedText)
    beautifiedText = re.sub('([(])\s+', '\g<1>', beautifiedText)
    return beautifiedText


def getRandomWord(): 
    """Fetch random word from system dictionary."""
    dictFile = "/usr/share/dict/words"
    assert os.path.exists(dictFile), "Could not find dictionary file at %s." % dictFile
    candidate = random.choice(open(dictFile).readlines()).rstrip('\n')
    return candidate

def queryEtymOnline(query, verbose=None):
    """Perform lookup on etymonline.com."""
    if verbose:
        sys.stdout.write("Querying etymonline.com for '%s'... " % query)

    r = requests.get("http://www.etymonline.com/index.php?search=" + query)
    soup = BeautifulSoup(r.content, convertEntities=BeautifulSoup.HTML_ENTITIES)

    try:
        hit = soup.dt.a.text
    except:
        raise NoResultsFound(query)

    if verbose:
        print "OK"

    etymology = beautify(soup.dd)
    return (hit, etymology)

def displayResults(hit, etymology):
    """Render results to STDOUT, with pretty whitespace."""
    t = Terminal()
    print(t.bold(hit))
    print(fill(etymology, width=t.width))
