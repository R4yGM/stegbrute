FROM rust:1.31
LABEL author="R4yan <https://github.com/R4yGM/stegbrute>"

WORKDIR /usr/src/stegbrute
COPY . .

RUN cargo install --path .

CMD ["stegbrute"]
