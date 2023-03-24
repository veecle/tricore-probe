use std::fmt::Debug;

use rpc_api::win_daemon::log::Record;

use super::pipe::Pipe;

pub fn spawn_piped<T: AsRef<str> + Debug + Send + 'static>(log_pipe: Pipe, target_context: T) {
    std::thread::spawn(move || {
        log::trace!("Reading logs from target_context {target_context:?} from pipe {log_pipe:?}");
        while let Ok(log_record) = ciborium::de::from_reader(log_pipe.open()) {
            let log_record: Record = log_record;
            let log_level: log::Level = log_record.level.into();
            if log_level <= log::max_level() {
                log::logger().log(
                    &log::Record::builder()
                        .args(format_args!("{}", log_record.message))
                        .level(log_level)
                        .target(&format!(
                            "{} <- {}",
                            log_record.target,
                            target_context.as_ref()
                        ))
                        .module_path(log_record.module_path.as_deref())
                        .file(log_record.file.as_deref())
                        .line(log_record.line)
                        .build(),
                );
            }
        }
    });
}
