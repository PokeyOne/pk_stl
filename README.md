- [Introduction](#sec-1)
  - [STL File Support](#sec-1-1)
- [Documentation](#sec-2)


# Introduction<a id="sec-1"></a>

pk\_stl is a Rust library for reading and writing STL files, and has no additional dependencies other than the standard library. It can read and write both ASCII and binary STL files.

## STL File Support<a id="sec-1-1"></a>

There are two main STL file formats: binary and ascii.

|       | Binary | ASCII |
|----- |------ |----- |
| Read  | Yes    | Yes   |
| Write | Yes    | Yes   |

Additionally, this library does not suppport any additional attributes attached to the triangles or model, but this features will be in a future release. If metadata from the header is needed, this library does provide access to the contents of the header.

# Documentation<a id="sec-2"></a>

For full documentaiton run:

```bash
cargo doc --open
```

Or visit <https://docs.rs/pk_stl/latest/pk_stl>

# Git Blame Ignore Revs

There is a `.git-blame-ignore-revs` file, which you can tell git about using:

``` bash
git config blame.ignoreRevsFile .git-blame-ignore-revs
```

This tells git blame to skip over commits in that file, such as large code
reformats which would otherwise polute the history.

