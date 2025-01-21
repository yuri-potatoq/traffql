use std::error::Error;

enum VmValue {
    Text(String),
    Integer(u32),
}

enum VmIdent {
    Name(String),
    Literal(String)
}

enum VmOPCodes {
    CALL(VmIdent),
    PUSH(VmValue),
    POP
}


struct Vm {
    
}


impl Vm {
    fn new() -> Self {
        Self{}
    }

    fn ingest(&self, instruction: Vec<VmOPCodes>) -> Result<(), String> {
        
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn execute_comparisson() {
        
    }
}