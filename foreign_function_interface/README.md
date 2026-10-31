# Installing Snappy Development Headers on Ubuntu

This guide covers how to install `snappy.h` and its associated development libraries on **Ubuntu Linux**.

## Installation Steps

To install the Snappy development package using the `apt` package manager, open your terminal and run the following commands:

```bash
sudo apt update
sudo apt install libsnappy-dev
```

### Included Packages
* **`libsnappy-dev`**: Installs the C++ header files (including `snappy.h`) required for compilation.
* **`libsnappy1v5`**: Installs the shared libraries needed to run software compiled with Snappy (automatically included as a dependency).

---

## Verification

After the installation completes, you can verify that the header file is present in your system's default include directory:

```bash
ls /usr/include/snappy.h
```

Alternatively, you can test if the compiler can successfully locate it:

```bash
echo '#include <snappy.h>' | g++ -E -x c++ - > /dev/null && echo "Found" || echo "Not found"
```
