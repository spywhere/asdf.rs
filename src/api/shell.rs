use std::io::Read;
use std::process::{Command, Stdio};
use std::time::Duration;

use wait_timeout::ChildExt;

pub struct SpawnOptions {
  pub working_dir: String,
  pub timeout: Duration,
}

pub struct ShellResult {
  pub exit_code: i32,
  pub stdout: String,
  pub stderr: String,
}

pub enum ShellError {
  ProcessError,
  ReadError,
  Timedout,
}

pub fn spawn<I>(
  command: &String,
  args: I,
  options: Option<SpawnOptions>,
) -> Result<ShellResult, ShellError>
where
  I: IntoIterator<Item = String>,
{
  let args = args.into_iter().collect::<Vec<String>>();
  let mut command = Command::new(command);

  let mut command = command.stdout(Stdio::piped()).stderr(Stdio::piped());

  if !args.is_empty() {
    command = command.args(args);
  }

  let mut timeout = Duration::MAX;

  if let Some(options) = options {
    timeout = options.timeout;
  }

  let Ok(mut child) = command.spawn() else {
    return Err(ShellError::ProcessError);
  };

  let status_code: Result<Option<i32>, ShellError> = match child
    .wait_timeout(timeout)
    .map_err(|_| ShellError::ProcessError)?
  {
    Some(status) => Ok(status.code()),
    None => {
      if matches!(child.kill(), Err(_)) {
        return Err(ShellError::ProcessError);
      }
      let Ok(status) = child.wait() else {
        return Err(ShellError::ProcessError);
      };
      Ok(status.code())
    }
  };

  let Some(status_code) = status_code? else {
    return Err(ShellError::Timedout);
  };

  let mut stdout = String::new();
  if let Some(mut sout) = child.stdout.take() {
    sout
      .read_to_string(&mut stdout)
      .map_err(|_| ShellError::ReadError)
      .map(|_| ())?
  }

  let mut stderr = String::new();
  if let Some(mut serr) = child.stderr.take() {
    serr
      .read_to_string(&mut stderr)
      .map_err(|_| ShellError::ReadError)
      .map(|_| ())?
  }

  Ok(ShellResult {
    exit_code: status_code,
    stdout,
    stderr,
  })
}
