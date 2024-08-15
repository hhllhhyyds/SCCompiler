#[derive(Clone, Copy, Debug)]
pub enum CompilerErrorLevel {
    Warning,
    Error,
}

#[derive(Clone, Copy, Debug)]
pub enum CompilerStage {
    Compile,
    Link,
}

#[derive(Clone, Debug)]
pub struct CompilerError {
    level: CompilerErrorLevel,
    stage: CompilerStage,
    content: String,
}

impl CompilerError {
    pub fn new(level: CompilerErrorLevel, stage: CompilerStage, content: &str) -> Self {
        Self {
            level,
            stage,
            content: String::from(content),
        }
    }

    pub fn process(&self) {
        let mut info = String::from("Compiler ");
        info += match self.stage {
            CompilerStage::Compile => "compile ",
            CompilerStage::Link => "link ",
        };
        info += "stage ";
        info += match self.level {
            CompilerErrorLevel::Warning => "warning: ",
            CompilerErrorLevel::Error => "error: ",
        };
        info += &self.content;

        match self.level {
            CompilerErrorLevel::Warning => println!("{info}"),
            CompilerErrorLevel::Error => panic!("{info}"),
        };
    }
}
