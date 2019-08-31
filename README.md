# 2D Texture Image Library
This is a library for working with two dimensional texture images in computer graphics 
applications. In particular, it provides a convenient set of abstractions for loading textures
onto the GPU, as well as optionally doing compile time code generation for embedding said
texture assets into a game binary.

## Usage
In order to use the library, add the following line to your `Cargo.toml` file:
```
[dependencies]
# ...
teximage2d = "0.1.3"
# ...
```
and then add the following line to either `lib.rs` or `main.rs` in your code
```rust
extern crate teximage2d;
```
and you are ready to go.

## Warning
Doing code generation to embed large binary assets is a bad idea. The resulting code fragment 
sizes get very large very fast since a compressed binary art asset is being decompressed, and
then being represented textually (as Rust code in this case). One is probably fine doing 
code generation with textual art assets such as wavefront obj files since they are already 
text, because the resulting code generated is comparable to the source.
