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
```

```

### Contributing

### License
This project is licensed under the MIT license
### Show your support
Leave a ⭐ if you like this project

***
Readme made with 💖 using [README Generator by Dhravya Shah](https://github.com/Dhravya/readme-generator)
