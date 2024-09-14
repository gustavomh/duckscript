use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use fsio::path::canonicalize_or;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "GetCanonicalPath")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["canonicalize".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("Path not provided.".to_string())
        } else {
            let path = canonicalize_or(&arguments.args[0], &arguments.args[0]);

            CommandResult::Continue(Some(path.to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
