extern crate texture;

use texture::TexImage2D;


pub fn to_rust_code(tex: &TexImage2D) -> String {
    let ir = generate_code(tex);
    synthesize_code(&ir)
}


struct TexImage2DIR {
    
}

fn generate_code(tex: &TexImage2D) -> TexImage2DIR { 
    unimplemented!("Code generation has not been implemented yet!");
}

fn synthesize_code(ir: &TexImage2DIR) -> String {
    unimplemented!("Code synthesis has not been implemented yet!");
}

