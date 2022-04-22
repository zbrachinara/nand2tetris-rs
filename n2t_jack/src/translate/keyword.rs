use strum_macros::EnumString;

#[derive(EnumString, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Arithmetic {
    Add,
    Sub,
    Neg,
    /// ```text
    /// @0
    /// M=M-1
    /// A=M
    /// D=M
    /// A=A-1
    ///
    /// M=M-D
    /// M=!M
    /// ```
    Eq,
    /// ```text
    /// // stack pointer access
    /// @0
    /// M=M-1
    /// A=M
    /// D=M
    /// A=A-1
    ///
    /// // write negative bit to D
    /// D=D-M
    /// @0b1000_0000_0000_0000
    /// D=D&A
    ///
    /// // write D to correct stack location
    /// @0
    /// A=M-1
    /// M=D
    /// ```
    Gt,
    /// ```text
    /// // stack pointer access
    /// @0
    /// M=M-1
    /// A=M
    /// D=M
    /// A=A-1
    ///
    /// // write negative bit to D
    /// D=M-D
    /// @0b1000_0000_0000_0000
    /// D=D&A
    ///
    /// // write D to correct stack location
    /// @0
    /// A=M-1
    /// M=D
    /// ```
    Lt,
    And,
    Or,
    Not,
}

#[derive(EnumString, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Stack {
    Push,
    Pop,
}
