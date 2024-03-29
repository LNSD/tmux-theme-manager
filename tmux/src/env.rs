use std::env;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum EnvError {
    #[error("$TMUX environment variable not present")]
    VarNotFound(#[from] std::env::VarError),

    #[error("Unknown $TMUX environment variable format")]
    VarUnknownFormat(String),
}

#[derive(Debug)]
pub struct Env {
    pub socket_path: PathBuf,
    pub server_pid: i32,
    pub session_idx: i32,
}

pub fn var() -> anyhow::Result<String, EnvError> {
    env::var("TMUX").map_err(|e| EnvError::VarNotFound(e))
}

#[allow(dead_code)]
pub fn parse_environ(var: &str) -> anyhow::Result<Env, EnvError> {
    let splitn: Vec<&str> = var.splitn(3, ",").collect();
    if splitn.len() != 3 {
        return Err(EnvError::VarUnknownFormat(var.to_owned()));
    }

    let socket_path = PathBuf::from(splitn[0]);
    let server_pid = splitn[1]
        .parse::<i32>()
        .map_err(|_| EnvError::VarUnknownFormat(var.to_owned()))?;
    let session_idx = splitn[2]
        .parse::<i32>()
        .map_err(|_| EnvError::VarUnknownFormat(var.to_owned()))?;

    Ok(Env {
        socket_path,
        server_pid,
        session_idx,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{EnvError, parse_environ};

    #[test]
    fn parse_valid_tmux_env_test() {
        let socket_path = PathBuf::from("/private/tmp/tmux-501/default");
        let tmux_env_var = String::from(format!("{},1023,0", socket_path.to_string_lossy()));

        let environ = parse_environ(&tmux_env_var);

        assert_matches!(environ, Ok(env) => {
            assert_eq!(env.socket_path, socket_path);
            assert_eq!(env.server_pid, 1023);
            assert_eq!(env.session_idx, 0);
        });
    }

    #[test]
    fn parse_invalid_var_format_test() {
        let tmux_env_var = String::from("/private/tmp/tmux-501/default 1023 0");

        let environ = parse_environ(&tmux_env_var);

        assert_matches!(environ, Err(EnvError::VarUnknownFormat(_)));
    }

    #[test]
    fn parse_invalid_server_pid_value_test() {
        let tmux_env_var = String::from("/private/tmp/tmux-501/default,JKL,0");

        let environ = parse_environ(&tmux_env_var);

        assert_matches!(environ, Err(EnvError::VarUnknownFormat(_)));
    }

    #[test]
    fn parse_invalid_session_id_value_test() {
        let tmux_env_var = String::from("/private/tmp/tmux-501/default,1023,JKL");

        let environ = parse_environ(&tmux_env_var);

        assert_matches!(environ, Err(EnvError::VarUnknownFormat(_)));
    }
}
