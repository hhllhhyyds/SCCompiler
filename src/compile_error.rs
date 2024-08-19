#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompileErrorLevel {
    Warning,
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompileStage {
    Compile,
    Link,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn compile_stage_error(content: &str) -> Self {
        Self::new(CompileErrorLevel::Error, CompileStage::Compile, content)
    }

    pub fn error_message(&self) -> String {
        self.content.clone()
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
