#[derive(Clone, Copy, Debug)]
pub enum CompileErrorLevel {
    Warning,
    Error,
}

#[derive(Clone, Copy, Debug)]
pub enum CompileStage {
    Compile,
    Link,
}

#[derive(Clone, Debug)]
pub struct CompileError {
    level: CompileErrorLevel,
    stage: CompileStage,
    content: String,
}

impl CompileError {
    pub fn new(level: CompileErrorLevel, stage: CompileStage, content: &str) -> Self {
        Self {
            level,
            stage,
            content: String::from(content),
        }
    }

    pub fn process(&self) {
        let mut info = String::from("SCCompiler ");
        info += match self.stage {
            CompileStage::Compile => "compile ",
            CompileStage::Link => "link ",
        };
        info += "stage ";
        info += match self.level {
            CompileErrorLevel::Warning => "warning: ",
            CompileErrorLevel::Error => "error: ",
        };
        info += &self.content;

        match self.level {
            CompileErrorLevel::Warning => println!("{info}"),
            CompileErrorLevel::Error => panic!("{info}"),
        };
    }
}
