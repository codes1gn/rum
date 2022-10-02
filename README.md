<div align="center">
<h1 align="center">rum</h1>
<br />
<img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg" /><br>
<br>
rust bindings and wrapper layer for mlir
</div>

***

### Installation
```
git clone git@github.com:codes1gn/llvm-project.git 
git checkout llvmorg-15.0.1 
./build_mlir.sh
./build_rum_bindings.sh
```
or build from docker
```
docker pull kernelci/clang-15
docker run -itd -v <local dir>:<target dir> -p <local port>:<target port> --network host --name <container-name> <image-name>
docker exec -it <container-name> /bin/bash

apt install build-essential,cmake,liblapack-dev,libopenblas-dev,curl,wget,ninja-build

./build_mlir.sh
./build_rum_bindings.sh
```

### Usage
All MLIR C API are form in rum_utils mod, if you wanna use it, config and add following code snippets in your code
```
// Cargo.toml
rum = "0.1"
```
```
// any mod
use rum::rum_utils::*;
```

### MLIR Manual
please refer to the official documentation of MLIR: mlir.llvm.org

### Contributing

### Thanks
Big thanks to related projects that inspired us
* The MLIR project: mlir.llvm.org/docs/
* the rust-bindgen project: github.com/rust-lang/rust-bindgen
* and mlir-sys project that shows a init demo on how to bind MLIR in rust: github.com/femtomc/mlir-sys

### License
This project is licensed under the MIT license
### Show your support
Leave a ⭐ if you like this project

