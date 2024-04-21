//! The `killport` library is designed to kill processes
//! listening on specified ports.
//!
//! The utility accepts a port number as input and attempts to
//! terminate any process listening on those port.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::kill_processes_by_port;
#[cfg(target_os = "macos")]
pub use macos::kill_processes_by_port;
#[cfg(target_os = "windows")]
pub use windows::kill_processes_by_port;

#[derive(Clone, Copy, Debug)]
pub enum KillPortSignalOptions {
    SIGKILL,
    SIGTERM,
}