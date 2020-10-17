FROM golang:1.13

RUN mkdir -p /app

WORKDIR /app

RUN apt-get update && apt-get install
RUN go get  github.com/R4yGM/netscanner

RUN netscanner
