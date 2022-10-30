- [Introduction](#sec-1)
  - [STL File Support](#sec-1-1)
- [Documentation](#sec-2)


# Introduction<a id="sec-1"></a>

pk\_stl is a Rust library for reading and writing STL files, and has no additional dependencies other than the standard library. It can read and write both ASCII and binary STL files (eventually; see section below).

## STL File Support<a id="sec-1-1"></a>

There are two main STL file formats: binary and ascii. This library will eventually support both formats full, however as this is in **very** early development, both are not fully supported yet.

|       | Binary | ASCII |
|----- |------ |----- |
| Read  | Yes    | No    |
| Write | No     | Yes   |

# Documentation<a id="sec-2"></a>

For full documentaiton run:

```bash
cargo doc --open
```

Or visit <https://docs.rs/pk_stl/latest/pk_stl>
