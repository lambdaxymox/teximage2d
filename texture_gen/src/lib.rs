extern crate texture;

use texture::TexImage2D;


/// Convert a texture image to a block of Rust code that can be
/// included into a computer program at build time.
pub fn to_rust_code(tex: &TexImage2D) -> String {
    let ir = generate_code(tex);
    synthesize_code(&ir)
}


#[derive(Copy, Clone, Debug, PartialEq)]
enum Token {
    SymUse,
    SymImportTexImage2D,
    SymLet,
    SymHeight,
    SymWidth,
    SymDepth,
    SymData,
    SymTypeU32,
    SymTypeRgba,
    SymTypeTexImage2D,
    SymTypeVec,
    SymRgbaNew,
    SymTexImage2DNew,
    SymMacroVec,
    Equals,
    Colon,
    DoubleColon,
    Semicolon,
    LBracket,
    RBracket,
    LCurlyBrace,
    RCurlyBrace,
    GreaterThan,
    LessThan,
    Comma,
    LParen,
    RParen,
    U8(u8),
    U32(u32),
    Newline,
    Whitespace(usize),
}

#[derive(Clone, Debug, PartialEq)]
struct TexImage2DIR { 
    data: Vec<Token> 
}

impl TexImage2DIR {
    fn new(data: Vec<Token>) -> TexImage2DIR {
        TexImage2DIR { data: data }
    }

    fn push(&mut self, item: Token) {
        self.data.push(item);
    }
}

/// Generate an import statement.
fn generate_imports(ir: &mut TexImage2DIR, indent: usize) {
    use Token::*;
    
    ir.push(Whitespace(indent));
    ir.push(SymUse);
    ir.push(Whitespace(1));
    ir.push(SymImportTexImage2D);
    ir.push(DoubleColon);
    ir.push(LCurlyBrace);
    ir.push(SymTypeRgba);
    ir.push(Comma);
    ir.push(Whitespace(1));
    ir.push(SymTypeTexImage2D);
    ir.push(RCurlyBrace);
    ir.push(Semicolon);
}

/// Generate the data set code for the texture image.
fn generate_data_code(ir: &mut TexImage2DIR, tex: &TexImage2D, indent: usize) {
    use Token::*;

    ir.push(Whitespace(indent));
    ir.push(SymLet);
    ir.push(Whitespace(1));
    ir.push(SymData);
    ir.push(Colon);
    ir.push(Whitespace(1));
    ir.push(SymTypeVec);
    ir.push(LessThan); ir.push(SymTypeRgba); ir.push(GreaterThan);
    ir.push(Whitespace(1));
    ir.push(Equals);
    ir.push(Whitespace(1));
    ir.push(SymMacroVec);
    ir.push(LBracket);
    ir.push(Newline);
    ir.push(Whitespace(indent));
    ir.push(Whitespace(indent));

    for pixel in tex.data.iter() {
        ir.push(SymTypeRgba);
        ir.push(DoubleColon);
        ir.push(SymRgbaNew);
        ir.push(LParen);
        ir.push(U8(pixel.r));
        ir.push(Comma);
        ir.push(Whitespace(1));
        ir.push(U8(pixel.g));
        ir.push(Comma);
        ir.push(Whitespace(1));
        ir.push(U8(pixel.b));
        ir.push(Comma);
        ir.push(Whitespace(1));
        ir.push(U8(pixel.a));
        ir.push(RParen);
        ir.push(Comma);
        ir.push(Whitespace(1));
    }

    ir.push(Newline);
    ir.push(Whitespace(indent));
    ir.push(RBracket);
    ir.push(Semicolon);
}

fn generate_height_code(ir: &mut TexImage2DIR, tex: &TexImage2D, indent: usize) {
    use Token::*;
    
    ir.push(Whitespace(indent));
    ir.push(SymLet);
    ir.push(Whitespace(1));
    ir.push(SymHeight);
    ir.push(Colon);
    ir.push(Whitespace(1));
    ir.push(SymTypeU32);
    ir.push(Whitespace(1));
    ir.push(Equals);
    ir.push(Whitespace(1));
    ir.push(U32(tex.height));
    ir.push(Semicolon);
}

fn generate_width_code(ir: &mut TexImage2DIR, tex: &TexImage2D, indent: usize) {
    use Token::*;
    
    ir.push(Whitespace(indent));
    ir.push(SymLet);
    ir.push(Whitespace(1));
    ir.push(SymWidth);
    ir.push(Colon);
    ir.push(Whitespace(1));
    ir.push(SymTypeU32);
    ir.push(Whitespace(1));
    ir.push(Equals);
    ir.push(Whitespace(1));
    ir.push(U32(tex.width));
    ir.push(Semicolon);
}

fn generate_depth_code(ir: &mut TexImage2DIR, tex: &TexImage2D, indent: usize) {
    use Token::*;
 
    ir.push(Whitespace(indent));
    ir.push(SymLet);
    ir.push(Whitespace(1));
    ir.push(SymDepth);
    ir.push(Colon);
    ir.push(Whitespace(1));
    ir.push(SymTypeU32);
    ir.push(Whitespace(1));
    ir.push(Equals);
    ir.push(Whitespace(1));
    ir.push(U32(tex.depth));
    ir.push(Semicolon);
}

/// Generate the type constructor invocation code.
fn generate_type_constructor_invocation(ir: &mut TexImage2DIR, indent: usize) {
    use Token::*;

    ir.push(Whitespace(indent));
    ir.push(SymTypeTexImage2D);
    ir.push(DoubleColon);
    ir.push(SymTexImage2DNew);
    ir.push(LParen);
    ir.push(SymWidth); ir.push(Comma); ir.push(Whitespace(1)); 
    ir.push(SymHeight); ir.push(Comma); ir.push(Whitespace(1)); 
    ir.push(SymData);
    ir.push(RParen);
}


/// Generate the Rust code expression block for constructing the
/// texture image at compile time.
fn generate_code(tex: &TexImage2D) -> TexImage2DIR {
    use Token::*;
    
    let mut ir = TexImage2DIR::new(vec![]);
    let indent = 4;
    // Start the code block.
    ir.push(LCurlyBrace);
    ir.push(Newline);

    // Generate the import statements.
    generate_imports(&mut ir, indent);
    ir.push(Newline);
    ir.push(Newline);

    generate_height_code(&mut ir, tex, indent);
    ir.push(Newline);

    generate_width_code(&mut ir, tex, indent);
    ir.push(Newline);

    generate_depth_code(&mut ir, tex, indent);
    ir.push(Newline);

    // Generate the data set.
    generate_data_code(&mut ir, tex, indent);
    ir.push(Newline);
    ir.push(Newline);

    // Generate the type constructor invocation.
    generate_type_constructor_invocation(&mut ir, indent);
    ir.push(Newline);

    // End the code block.    
    ir.push(RCurlyBrace);

    ir
}

fn synthesize_token(token: Token) -> String {
    use Token::*;
    match token {
        SymUse => format!("{}", "use"),
        SymImportTexImage2D => format!("{}", "teximage2d"),
        SymLet => format!("{}", "let"),
        SymHeight => format!("{}", "height"),
        SymWidth => format!("{}", "width"),
        SymDepth => format!("{}", "depth"),
        SymData => format!("{}", "data"),
        SymTypeU32 => format!("{}", "u32"),
        SymTypeRgba => format!("{}", "Rgba"),
        SymTypeTexImage2D => format!("{}", "TexImage2D"),
        SymTypeVec => format!("{}", "Vec"),
        SymRgbaNew => format!("{}", "new"),
        SymTexImage2DNew => format!("{}", "from_rgba_data"),
        SymMacroVec => format!("{}", "vec!"),
        Equals => format!("{}", "="),
        Colon => format!("{}", ":"),
        DoubleColon => format!("{}", "::"),
        Semicolon => format!("{}", ";"),
        LBracket => format!("{}", "["),
        RBracket => format!("{}", "]"),
        LCurlyBrace => format!("{}", "{"),
        RCurlyBrace => format!("{}", "}"),
        GreaterThan => format!("{}", ">"),
        LessThan => format!("{}", "<"),
        Comma => format!("{}", ","),
        LParen => format!("{}", "("),
        RParen => format!("{}", ")"),
        U8(number) => format!("{:#02X}", number),
        U32(number) => format!("{}", number),
        Newline => format!("{}", "\n"),
        Whitespace(number) => format!("{:width$}", "", width = number),
    }
}

fn synthesize_code(ir: &TexImage2DIR) -> String {
    let mut fragment = String::new();
    for token in ir.data.iter() {
        fragment.push_str(&synthesize_token(*token));
    }

    fragment
}

#[cfg(test)]
mod tests {

}