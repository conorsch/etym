class NoResultsFound(Exception):
    def __init__(self, value: str) -> None:
        self.value = "Found no hits for query '%s'." % value

    def __repr__(self) -> str:
        return "<NoResultsFound> {}".format(self.value)
