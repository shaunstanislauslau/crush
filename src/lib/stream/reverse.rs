use crate::lang::table::Row;
use crate::lang::execution_context::ExecutionContext;
use crate::lang::errors::{CrushResult, error};
use crate::lang::stream::{Readable, ValueSender};

pub fn run(
    input: &mut dyn Readable,
    sender: ValueSender,
) -> CrushResult<()> {
    let output = sender.initialize(input.types().clone())?;
    let mut q: Vec<Row> = Vec::new();
    while let Ok(row) = input.read() {
        q.push(row);
    }
    while !q.is_empty() {
        output.send(q.pop().unwrap())?;
    }
    Ok(())
}

pub fn perform(context: ExecutionContext) -> CrushResult<()> {
    match context.input.recv()?.readable() {
        Some(mut input) => run(input.as_mut(), context.output),
        None => error("Expected a stream"),
    }
}
