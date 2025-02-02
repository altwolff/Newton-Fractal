# Newton Fractal
This Rust project generates a Newton fractal using the Newton-Raphson method of the function $f(z) = z^3 - 1$, where the fractal's colour scheme is based on the root each pixel converges to.    

The generated fractal is saved as a PNG image (`newton.png`), and the program offers customisation options for the size and iterations of the fractal.

## Requirements
* Rust (https://www.rust-lang.org/tools/install)
  
  Linux:
  ```
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
* Dependencies: `num` and `image`

## Installation
1. Clone repository:
```
git clone https://github.com/altwolff/Newton-Fractal.git
cd Newton-Fractal
```
2. Build project:
```
cargo build --release
```
3. Run project:
```
cargo run
```

## Customisation
The following can be edited:
```
let width = 800;
let height = 800;
let max_iterations = 1000;
```
The `width` and `height` parameters alter the image dimensions.    

A higher `max_iterations` value will improve the fractal's detail, but at the cost of computation time.    

Moreover, colours associated with each root may also be edited:
```
let root1_color = Rgb([77, 182, 172]);
let root2_color = Rgb([255, 138, 101]);
let root3_color = Rgb([149, 117, 205]);
let default_color = Rgb([0, 0, 0]);
```


## Preview
![Generated image](newton.png)
