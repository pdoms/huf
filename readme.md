# Huf - Huffman Coding Based Compression of Files

Ascii text compressed with Huffmann Coding. Written in rust. 
This was just for fun.

### General
Have [rustc](https://rustup.rs/) installed and make sure the [build.sh](./build.sh)
is executable.

### To build
```console
$ ./build.sh build [out file name]
```
You can optionally pass a name for the output binary file. It defauls to `huf`.

### To Run
Just execute the binary, depending on your os.

### To Test 
```console
$ ./build.sh test [out file name]
```
You can optionally pass an out file name. It defauls to `huf_test`.
Some tests run only to make sure the program is not breaking.

### On build.sh
The script also has a cleanup function. 
```console
$ ./build.sh clean
```
## Todos
- [ ] Performance tests
- [ ] Optimization
- [ ] Cli Improvements


**Remeber this project is just for fun**
**Remeber coding should be fun**


