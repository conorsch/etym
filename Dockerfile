FROM debian:buster AS builder
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y dh-virtualenv python3-dev make git build-essential debhelper devscripts equivs

COPY . /code
WORKDIR /code
RUN ls -l
RUN make deb
RUN dpkg -i /*.deb
RUN etym --help

FROM python:buster
COPY --from=builder /*.deb /tmp/
RUN apt-get install -y -f /tmp/etym*.deb
RUN etym --help
