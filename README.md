![](https://api.travis-ci.org/R4yGM/StegBrute.svg)
![](https://img.shields.io/docker/cloud/build/r4yan/stegbrute)
# stegbrute
stegbrute is a fast steganography brute-force tool written in Rust using also threads to achieve a faster execution 

<p align="center">
  <img src="https://i.imgur.com/zGFolUt.png" >
</p>

# Dependencies

Stegbrute cannot run without **steghide**!, to install steghide run :

```bash
apt-get install -y steghide
```

if you are not in a debian distribution you can download it from steghide [website](http://steghide.sourceforge.net/)

# Installation
stegbrute can be installed in different ways:

## **Cargo**

<img src="https://community.kde.org/images.community/thumb/5/5e/Rust-logo-512x512-blk.png/300px-Rust-logo-512x512-blk.png" width=35 height=35>

 throught [cargo](https://github.com/rust-lang/cargo) (Rust package manager)
 
 if you don't have cargo you can install it either from apt or by downloading Rust lang
```bash
cargo install stegbrute
```
**this will work for every platform**

## Debian distributions

<img src="https://cdn0.iconfinder.com/data/icons/flat-round-system/512/debian-512.png" width=35 height=35>

if you have ubuntu/kali or other debian distributions you can install the .deb file you find on the [releases](https://github.com/R4yGM/stegbrute/releases) section, then unpack the file and run it
```bash
wget https://github.com/R4yGM/stegbrute/releases/download/0.1.1/stegbrute_0.1.1_amd64.deb &&
dpkg --install stegbrute_0.1.1_amd64.deb
```

## **Docker**

<img src="https://cdn3.iconfinder.com/data/icons/logos-and-brands-adobe/512/97_Docker-512.png" width=35 height=35>

  if you don't have docker installed you can follow their [guide](https://docs.docker.com/engine/install/)
  
 first you have to pull the docker image (only **4.93 MB**) from the docker registry, you can see it [here](https://hub.docker.com/r/r4yan/stegbrute), if you don't want to pull the image you can also clone the repository and then build the image from the Dockerfile
 ```bash
docker pull r4yan/stegbrute:latest
  ```
  you can also decide to pull different images by replacing 'latest' with a stegbrute version, ex.
  ```bash
docker pull r4yan/stegbrute:0.1.0
  ```
  
  if you don't want to pull the image you can download/copy stegbrute Dockerfile that can be found [here](https://github.com/R4yGM/stegbrute/blob/main/Dockerfile) and then build the image from the Dockerfile
  
  then if you want to launch the container you have to first create a volume to share your files to the container
  
  ```bash
  docker volume create --name stegbrute_data
  ``` 
  then move or copy the files you want to use for stegbrute inside the volume folder wich usually is here `/var/lib/docker/volumes/stegbrute_data/_data` by just doing
  ```bash
  cp wordlist.txt /var/lib/docker/volumes/stegbrute_data/_data && cp file.jpg /var/lib/docker/volumes/stegbrute_data/_data
  ```
  and now run stegbrute
  ```bash
  docker run -v stegbrute_data:/stegbrute_data -it --rm --name stegbrute r4yan/stegbrute:latest <options>
  ```
  replace the `<options>` with the options/arguments you want to give to stegbrute,
  once you did everything you don't have to pull/build the image again only if there are new updates or features
  
  **Always save your results inside the volume and not in the container because then the results will be deleted! you can save them by adding this option `-x /$VOLUME_NAME/results.txt` or `--extract-file /$VOLUME_NAME/results.txt`** 
 
 if you added this and did everything correctly at the end of every attack you'd find the results inside the folder `/var/lib/docker/volumes/stegbrute_data/_data`
  
  
  **this will work for every platform**
  
  ## Executable
  you can also download the already compiled programn and then execute it, example :
  ```bash
wget https://github.com/R4yGM/stegbrute/releases/download/0.1.1/stegbrute && chmod +x stegbrute
mv stegbrute /usr/local/bin/
```

# Usage

stegbrute is very simple to use and it gives you many options, you can view the program help with the -h or --help option

```bash
============================================================
     ____  _             ____             _
    / ___|| |_ ___  __ _| __ ) _ __ _   _| |_ ___
    \___ \| __/ _ \/ _` |  _ \| '__| | | | __/ _ \
     ___) | ||  __/ (_| | |_) | |  | |_| | ||  __/
    |____/ \__\___|\__, |____/|_|   \__,_|\__\___|
                   |___/

StegBrute v0.1.1 - By R4yan
https://github.com/R4yGM/StegBrute

StegBrute 0.1.1
R4yan <yessou.rayan@gmail.com>
Steganography bruteforce tool

USAGE:
    stegbrute [FLAGS] [OPTIONS] --file-name <file-name> --wordlist <wordlist>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    shows every try the program does

OPTIONS:
    -x, --extract-file <extract-file>    the file name path where you want to write the results [default:
                                         stegbrute_results.txt]
    -f, --file-name <file-name>          the file name path you want to crack
    -t, --threads <threads>              number of threads to bruteforce the file [default: 3]
    -w, --wordlist <wordlist>            path of the wordlist
```

for example :

<p align="center">
<a href="https://asciinema.org/a/5YUpQhY76MQE6vXDIVNNyK9T7" target="_blank"><img src="https://asciinema.org/a/5YUpQhY76MQE6vXDIVNNyK9T7.svg" /></a>
</p>

Options :
- `-x` or `--extract-file` with `<file_name>` will save the results of the extracted data into the file_name, if no file is specified stegbrute will save your results inside ./stegbrute_results.txt file

- `-t` or `--threads` with `<number_of_threads>` will launch a number of programs bruteforcing the file simultaneously, incrementing the number of threads doesn't always mean this will run more faster it all depends on how many threads your machine can handle

- `-f` or `--file_name` with `<file_name>` the file name that stegbrute is going to attack, must be one of these supported formats : JPEG, BMP, WAV or AU

- `-w` or `--wordlist` with `<wordlist>` the file where stegbrute is going to take the passwords line by line and then start trying them to the file you want to crack, if you don't have one you can install for example [rockyou.txt](https://github.com/praetorian-inc/Hob0Rules/blob/master/wordlists/rockyou.txt.gz)

# Benchmark
stegbrute benchmark on different wordlists using 3 threads

| Wordlist passwords   | Time  |  
|---|---|
| 100   | 841.12ms  |  
| 1000  | 8.57s  |   
| 10000 | 77.79s |
| 100000 | 775.93s  |  
