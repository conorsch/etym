DEFAULT_GOAL := "all"

.PHONY: all
all: lint test

.PHONY: test
test:
	pytest

.PHONY: lint
lint:
	flake8 --max-line-length 100 .
	black --line-length 100 .
	mypy --strict --ignore-missing-imports .


.PHONY: clean
clean:
	git clean -fdx
