use crate::lib::ExecutionContext;
use crate::errors::CrushResult;
use crate::{
    data::Row,
    data::ValueType,
    stream::{OutputStream},
    data::Value,
};
use psutil::process::State;
use crate::lib::command_util::{create_user_map, UserMap};
use users::uid_t;
use crate::data::ColumnType;
use chrono::Duration;

fn state_name(s: psutil::process::State) -> &'static str {
    match s {
        State::Running => "Running",
        State::Sleeping => "Sleeping",
        State::Waiting => "Waiting",
        State::Stopped => "Stopped",
        State::Traced => "Traced",
        State::Paging => "Paging",
        State::Dead => "Dead",
        State::Zombie => "Zombie",
        State::Idle => "Idle",
    }
}

pub fn run(output: OutputStream) -> CrushResult<()> {
    let users = create_user_map();

    for proc in &psutil::process::all().unwrap() {
        output.send(Row::new(vec![
                Value::Integer(proc.pid as i128),
                Value::Integer(proc.ppid as i128),
                Value::text(state_name(proc.state)),
                users.get_name(proc.uid as uid_t),
                Value::Duration(Duration::microseconds((proc.utime*1000000.0) as i64)),
                Value::text(
                    proc.cmdline_vec().unwrap_or_else(|_| Some(vec!["<Illegal name>".to_string()]))
                        .unwrap_or_else(|| vec![format!("[{}]", proc.comm)])[0]
                        .as_ref()),
            ]))?;
    }
    Ok(())
}

pub fn perform(context: ExecutionContext) -> CrushResult<()> {
    let output = context.output.initialize(vec![
        ColumnType::named("pid", ValueType::Integer),
        ColumnType::named("ppid", ValueType::Integer),
        ColumnType::named("status", ValueType::Text),
        ColumnType::named("user", ValueType::Text),
        ColumnType::named("cpu", ValueType::Duration),
        ColumnType::named("name", ValueType::Text),
    ])?;
    run(output)
}