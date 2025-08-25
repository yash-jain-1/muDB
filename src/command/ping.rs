use crate::resp::types::RespType;

use super::CommandError;

// Represents the PING command in MuDB.
// The PING command can optionally include a message to be echoed back
struct Ping {
    message: Option<String>,
}

impl Ping {
    /// Creates a new `Ping` instance from the given arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - A vector of `RespType` representing the arguments to the PING command.
    ///
    /// # Returns
    ///
    /// * `Ok(Ping)` if parsing succeeds.
    /// * `Err(CommandError)` if parsing fails.
    
    pub fn with_args(args: Vec<RespType>) -> Result<Ping, CommandError> {
        if args.len() == 0 {
            return Ok(Ping { msg: None });
        }

        let msg = match &args[0] {
            RespType::BulkString(s) => s.clone(),
            _ => return Err(CommandError::Other(String::from("Invalid message"))),
        };

        Ok(Ping { msg: Some(msg) })
    }

    pub fn apply(&self) -> RespType {
        if let Some(msg) = &self.msg {
            RespType::BulkString(msg.to_string())
        } else {
            RespType::SimpleString(String::from("PONG"))
        }
    }
}