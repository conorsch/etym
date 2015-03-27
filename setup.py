from distutils.core import setup

setup(
    name = 'etym',
    packages = ['etym'],
    version = '0.0.2',
    description = 'Command-line interface for EtymOnline.com.',
    author = 'Conor Schaefer',
    author_email = 'conor.schaefer@gmail.com',
    url = 'https://github.com/conorsch/etym',
    download_url = 'https://github.com/conorsch/etym/tarball/0.0.1',
    scripts=['bin/etym'],
    install_requires=[
        'BeautifulSoup>=3.2.1',
        'blessings>=1.6',
        'docopt>=0.6.2',
        'requests>=2.6.0',
        ],
    keywords = ['etymology', 'language', 'words', 'dictionary'],
    classifiers = [],
)
