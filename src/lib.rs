use robson_compiler::{
  data_struct::{IError, TypedByte},
  interpreter::Interpreter,
};

pub fn run_rbsn(buffer: &[u8]) -> Result<Vec<TypedByte>, IError> {
  let mut interpreter =
    Interpreter::new(buffer, robson_compiler::infra::RunInfra::new())?;
  interpreter.run_buffer()?;
  Ok(interpreter.stack.vec)
}
