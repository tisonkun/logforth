use crate::append::file::Message;
use crossbeam_channel::{Receiver, RecvError, TryRecvError};
use std::io;
use std::io::Write;

pub(super) struct Worker<T: Write + Send + 'static> {
    writer: T,
    receiver: Receiver<Message>,
    shutdown: Receiver<()>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum WorkerState {
    Empty,
    Disconnected,
    Continue,
    Shutdown,
}

impl<T: Write + Send + 'static> Worker<T> {
    pub(crate) fn new(writer: T, receiver: Receiver<Message>, shutdown: Receiver<()>) -> Worker<T> {
        Self {
            writer,
            receiver,
            shutdown,
        }
    }

    fn recv(&mut self) -> io::Result<WorkerState> {
        match self.receiver.recv() {
            Ok(Message::Record(record)) => {
                self.writer.write_all(&record)?;
                Ok(WorkerState::Continue)
            }
            Ok(Message::Shutdown) => Ok(WorkerState::Shutdown),
            Err(RecvError) => Ok(WorkerState::Disconnected),
        }
    }

    fn try_recv(&mut self) -> io::Result<WorkerState> {
        match self.receiver.try_recv() {
            Ok(Message::Record(record)) => {
                self.writer.write_all(&record)?;
                Ok(WorkerState::Continue)
            }
            Ok(Message::Shutdown) => Ok(WorkerState::Shutdown),
            Err(TryRecvError::Empty) => Ok(WorkerState::Empty),
            Err(TryRecvError::Disconnected) => Ok(WorkerState::Disconnected),
        }
    }

    pub(super) fn work(&mut self) -> io::Result<WorkerState> {
        let mut worker_state = self.recv()?;

        while worker_state == WorkerState::Continue {
            worker_state = self.try_recv()?;
        }

        self.writer.flush()?;
        Ok(worker_state)
    }

    pub(super) fn make_thread(mut self, name: String) -> std::thread::JoinHandle<()> {
        std::thread::Builder::new()
            .name(name)
            .spawn(move || {
                loop {
                    match self.work() {
                        Ok(WorkerState::Continue) | Ok(WorkerState::Empty) => {}
                        Ok(WorkerState::Shutdown) | Ok(WorkerState::Disconnected) => {
                            let _ = self.shutdown.recv();
                            break;
                        }
                        Err(err) => {
                            eprintln!("failed to write log: {err}");
                        }
                    }
                }
                if let Err(err) = self.writer.flush() {
                    eprintln!("failed to flush: {err}");
                }
            })
            .expect("failed to spawn the non-blocking rolling file writer thread")
    }
}
