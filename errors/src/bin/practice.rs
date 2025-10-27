// fn get_value(opt: Option<i32>) -> i32 {
//     opt.unwrap_or_else(|| 0)
// }
// fn main() {
//     let value = get_value(Some(10));
//     println!("Value: {}", value);

//     let default_value = get_value(None);
//     println!("Default value: {}", default_value);
// }

// #[derive(Debug)]
// enum NetworkError {
//     Disconnected,
//     Timeout,
// }
// #[derive(Debug)]
// enum CustomError {
//     NotFound,
//     Network(NetworkError),
// }
// impl From<NetworkError> for CustomError {
//     fn from(err: NetworkError) -> Self {
//         CustomError::Network(err)
//     }
// }
// fn simulate_network() -> Result<(), NetworkError> {
//     Err(NetworkError::Disconnected)
// }
// fn main() -> Result<(), CustomError> {
//     simulate_network()?;
//     Ok(())
// }

// enum JobStatus {
//     Applied,
//     Interviewing,
//     Offered,
//     Rejected,
// }
// struct Candidate {
//     name: String,
//     status: JobStatus,
// }
// fn main() {
//     let candidate = Candidate {
//         name: String::from("Alice"),
//         status: JobStatus::Interviewing,
//     };
//     match candidate.status {
//         JobStatus::Applied => println!("{} has applied.", candidate.name),
//         JobStatus::Interviewing => println!("{} is interviewing.", candidate.name),
//         JobStatus::Offered => println!("{} has been offered the job.", candidate.name),
//         JobStatus::Rejected => println!("{} has been rejected.", candidate.name),
//     }
// }

use env_logger;
use log::{error, info, warn};
fn main() {
    env_logger::init();
    info!("This is an info message");
    warn!("This is a warning");
    error!("This is an error");
}
