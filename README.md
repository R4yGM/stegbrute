![](https://api.travis-ci.org/R4yGM/StegBrute.svg)
# stegbrute
stegbrute is a steganography brute-force tool written in Rust to achieve a faster execution using also threads

<p align="center">
  <img src="https://i.imgur.com/zGFolUt.png" >
</p>


# Installation
stegbrute can be installed in different ways:

<img src="https://community.kde.org/images.community/thumb/5/5e/Rust-logo-512x512-blk.png/300px-Rust-logo-512x512-blk.png" width=35 height=35>

- **Cargo**:
 throught [cargo](https://github.com/rust-lang/cargo) (Rust package manager)
 
 if you don't have cargo you can install it either from apt or by downloading Rust lang
```bash
cargo install stegbrute
```
**this will work for every platform**

<img src="https://cdn3.iconfinder.com/data/icons/logos-and-brands-adobe/512/97_Docker-512.png" width=35 height=35>

- **Docker**:
  if you don't have docker installed you can follow their [guide](https://docs.docker.com/engine/install/)
  
 first you have to pull the docker image from the docker registry, you can see it [here](https://hub.docker.com/r/r4yan/stegbrute), if you don't want to pull the image you can also clone the repository and then build the image from the Dockerfile
```bash
docker pull r4yan/stegbrute:latest
  ```
  you can also decide to pull different images by replacing 'latest' with a stegbrute version
  
  then you can run the program by launching a container and then autoremoving it 
  ```bash
  docker run -it --rm --name stegbrute r4yan/stegbrute:latest <options>
  ```
  replace the `<options>` with the options/arguments you want to give to stegbrute
  
  **this will work for every platform**
# Usage
