DEFAULT_GOAL := "all"

.PHONY: all
all: lint test

.PHONY: test
test:
	pytest -v

.PHONY: lint
lint:
	flake8 --max-line-length 100 --exclude debian/
	black --line-length 100 .
	mypy --strict --ignore-missing-imports .

.PHONY: clean
clean:
	git clean -fdx

.PHONY: deb
deb:
	dpkg-buildpackage -us -uc
	mv ../etym_*_amd64.deb dist/
	find dist/ -type f -iname 'etym*.deb' | sort -n

.PHONY: reprotest
reprotest:
	reprotest -c 'make deb' . 'dist/*.deb' --min-cpus 4 \
		--variations "-all, +fileordering"

# PASS: fileordering, kernel, aslr, user_group
# FAIL: build_path, time
# "+environment, +build_path, +kernel, +aslr, +num_cpus, +time, +user_group, +fileordering, +domain_host, +home, +locales, +exec_path, +timezone, +umask"
