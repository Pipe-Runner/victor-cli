# Victor
A vector processor written in Rust as part of the exercise in IE500217 (Computer Graphics) course.

# Setup
To use the processor, you need to make sure you have `rust` and `cargo` installed. Follow this [link](https://www.rust-lang.org/tools/install) for more details.

# Usage
The program uses a CLI based menu to let the user choose between different operations. To run the program simply issue the following command: 
 
`cargo run`

This will bring in the following menu:

```
Please select an option:
1. Add two vectors
2. Subtract two vectors
3. Multiply a scalar to a vector
4. Angle between two vectors
5. Dot product of two vectors
6. Cross product of two 3D vectors
7. Generate 3D basis from one vector
8. Generate 3D basis from two vectors
9. Exit
```

**Note:** Vector inputs are taken as space separated numbers. For example for `2i + 3j + 4k`, we would input `2 3 4`.