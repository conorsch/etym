class NoResultsFound(Exception):
    def __init__(self, value):
        self.value = "Found no hits for query '%s'." % value

    def __str__(self):
        return repr(self.value)

