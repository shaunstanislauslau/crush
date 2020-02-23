use crate::scope::Scope;
use crate::errors::CrushResult;
use crate::lang::{Value, SimpleCommand};

mod set;
mod r#let;
mod unset;
mod env;

pub fn declare(root: &Scope) -> CrushResult<()> {
    let env = root.create_namespace("var")?;
    root.uses(&env);
    env.declare_str("let", Value::Command(SimpleCommand::new(r#let::perform)))?;
    env.declare_str("set", Value::Command(SimpleCommand::new(set::perform)))?;
    env.declare_str("unset", Value::Command(SimpleCommand::new(unset::perform)))?;
    env.declare_str("env", Value::Command(SimpleCommand::new(env::perform)))?;
    env.readonly();
    Ok(())
}