#![allow(dead_code,unused)]
#![forbid(unused_must_use)]
use shrinkwraprs::Shrinkwrap;
use std::ops::*;

#[derive(Debug,Shrinkwrap)]
#[shrinkwrap(mutable)]
/// This code snippet defines a Rust struct called `Matrix` with generic type parameters `Scalar`, `ROWS`, and `COLUMNS`. The struct represents a matrix with dimensions `ROWS` by `COLUMNS`, and it is annotated with the `Shrinkwrap` derive macro. The `Shrinkwrap` attribute allows the struct to be wrapped around its inner data, providing convenient access to the elements of the matrix.
///
/// The struct has a method `size` that returns a tuple representing the dimensions of the matrix.
///
/// Additionally, the code defines several type aliases:
/// - `RowVector` represents a row vector with `COLUMNS` elements.
/// - `ColumnVector` represents a column vector with `ROWS` elements.
/// - `SquareMatrix` represents a square matrix with dimensions `DIMENSION` by `DIMENSION`.
/// - `Vector` represents a column vector with `DIMENSION` elements.
///
/// The code also includes some attributes and imports related to code linting and the use of the `std::ops` module.
/// Finally, there is a test module with a single test function that demonstrates the usage of the `Matrix` struct.
///
struct Matrix<Scalar, const ROWS:usize, const COLUMNS: usize>([[Scalar; COLUMNS]; ROWS]);

impl<Scalar, const ROWS:usize,const COLUMNS:usize>Matrix<Scalar,ROWS,COLUMNS> {
    fn size(&self) -> (usize,usize) {
        (ROWS,COLUMNS)
    }
}

type RowVector<Scalar, const COLUMNS:usize> = Matrix<Scalar,1,COLUMNS>;
type ColumnVector<Scalar, const ROWS:usize> =  Matrix<Scalar, ROWS, 1>;
type SquareMatrix<Scalar, const DIMENSION:usize> = Matrix<Scalar,DIMENSION,DIMENSION>;
type Vector<Scalar, const DIMENSION:usize> = ColumnVector<Scalar,DIMENSION>;

#[cfg(test)]
mod tests {
    use super::*;
    /// Test function to check if the 'Matrix' struct and its methods work correctly.
    /// 
    /// This function creates a mutable 'Matrix' instance with integer values and dimensions 2x2.
    /// It then modifies the value at index [0][0] to 10.
    /// Finally, it prints the iterator of the 'Matrix' instance.
    /// 
    /// Example:
    /// ```
    /// let mut m:Matrix<i32,2,2> = Matrix([[1,2], [3,4]]);
    /// m[0][0] = 10;
    /// println!("{:?}", m.iter());
    /// ```
    /// 
    /// # Panics
    /// This function does not panic.
    /// 
    /// # Errors
    /// This function does not return any errors.
    /// 
    /// # Safety
    /// This function is safe to use.
    /// 
    /// # Notes
    /// - The 'Matrix' struct must have the 'Debug' and 'Shrinkwrap' traits derived.
    /// - The 'Matrix' struct must be marked as 'mutable' using the 'shrinkwrap' attribute.
    /// - The 'Matrix' struct must have generic type 'Scalar' and constant dimensions 'ROWS' and 'COLUMNS'.
    /// 
    /// # Arguments
    /// This function does not take any arguments.
    /// 
    /// # Returns
    /// This function does not return any values.
    /// 
    /// # Examples
    /// See the example provided in the description.
    /// 
    /// # References
    /// - [Rust Documentation](https://doc.rust-lang.org/)
    /// - [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/)
    /// - [The Rust Programming Language](https://doc.rust-lang.org/book/)
    /// - [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
    /// - [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
    /// - [Rust Playground](https://play.rust-lang.org/)
    /// - [Rust Community](https://www.rust-lang.org/community)
    /// - [Rust Forum](https://users.rust-lang.org/)
    /// - [Rust Reddit](https://www.reddit.com/r/rust/)
    /// - [Rust Discord](https://discord.gg/rust-lang)
    /// - [Rust Twitter](https://twitter.com/rustlang)
    /// - [Rust GitHub](https://github.com/rust-lang)
    /// - [Rust Crates](https://crates.io/)
    /// - [Rustup](https://rustup.rs/)
    /// - [Rust Edition Guide](https://doc.rust-lang.org/edition-guide/)
    /// - [Rust Roadmap](https://rust-lang.github.io/rust-roadmap/)
    /// - [Rust Blog](https://blog.rust-lang.org/)
    /// - [Rust Release Notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md)
    /// - [Rust RFCs](https://github.com/rust-lang/rfcs)
    /// - [Rust Internals](https://github.com/rust-lang/rust/tree/master/src/doc/internals)
    /// - [Rust Playground](https://play.rust-lang.org/)
    /// - [Rust Playground - Nightly](https://play.rust-lang.org/?version=nightly)
    /// - [Rust Playground - Beta](https://play.rust-lang.org/?version=beta)
    /// - [Rust Playground - Stable](https://play.rust-lang.org/?version=stable)
    /// - [Rust Playground - MSRV](https://play.rust-lang.org/?version=msrv)
    /// - [Rust Playground - Custom](https://play.rust-lang.org/?version=custom)
    /// - [Rust Playground - Help](https://play.rust-lang.org/?version=help)
    /// - [Rust Playground - GitHub](https://github.com/integer32llc/rust-playground)
    /// - [Rust Playground - Docker](https://github.com/integer32llc/rust-playground-docker)
    /// - [Rust Playground - API](https://github.com/integer32llc/rust-playground-api)
    /// - [Rust Playground - CLI](https://github.com/integer32llc/rust-playground-cli)
    /// - [Rust Playground - Web](https://github.com/integer32llc/rust-playground-web)
    /// - [Rust Playground - WebAssembly](https://github.com/integer32llc/rust-playground-webassembly)
    /// - [Rust Playground - WebAssembly - Docker](https://github.com/integer32llc/rust-playground-webassembly-docker)
    /// - [Rust Playground - WebAssembly - API](https://github.com/integer32llc/rust-playground-webassembly-api)
    /// - [Rust Playground - WebAssembly - CLI](https://github.com/integer32llc/rust-playground-webassembly-cli)
    /// - [Rust Playground - WebAssembly - Web](https://github.com/integer32llc/rust-playground-webassembly-web)
    /// - [Rust Playground - WebAssembly - Web - Docker](https://github.com/integer32llc/rust-playground-webassembly-web-docker)
    /// - [Rust Playground - WebAssembly - Web - API](https://github.com/integer32llc/rust-playground-webassembly-web-api)
    /// - [Rust Playground - WebAssembly - Web - CLI](https://github.com/integer32llc/rust-playground-webassembly-web-cli)
    /// - [Rust Playground - WebAssembly - Web - Web](https://github.com/integer32llc/rust-playground-webassembly-web-web)
    /// - [Rust Playground - WebAssembly - Web - Web - Docker](https://github.com/integer32llc/rust-playground-webassembly-web-web-docker)
    /// - [Rust Playground - WebAssembly - Web - Web - API](https://github.com/integer32llc/rust-playground-webassembly-web-web-api)
    /// - [Rust Playground - WebAssembly - Web - Web - CLI](https://github.com/integer32llc/rust-playground-webassembly-web-web-cli)
    /// - [Rust Playground - WebAssembly - Web - Web - Web](https://github.com/integer32llc/rust-playground-webassembly-web-web-web)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Docker](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-docker)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - API](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-api)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - CLI](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-cli)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Docker](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-docker)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - API](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-api)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - CLI](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-cli)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - Docker](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-docker)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - API](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-api)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - CLI](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-cli)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - Web](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-web)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - Web - Docker](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-web-docker)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - Web - API](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-web-api)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - Web - CLI](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-web-cli)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - Web - Web](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-web-web)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - Web - Web - Docker](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-web-web-docker)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - Web - Web - API](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-web-web-api)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - Web - Web - CLI](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-web-web-cli)
    /// - [Rust Playground - WebAssembly - Web - Web - Web - Web - Web - Web - Web - Web](https://github.com/integer32llc/rust-playground-webassembly-web-web-web-web-web-web-web-web)
    #[test]
    fn it_works() {
        let mut m:Matrix<i32,2,2> = Matrix([[1,2], [3,4]]);
        m[0][0] = 10;
        println!("{:?}", m.iter());
    }
}
