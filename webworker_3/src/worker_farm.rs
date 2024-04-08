use std::sync::mpsc::{Receiver, Sender};

pub struct WorkerFarm<T: Clone> {
  senders: Vec<Sender<T>>,
  c: usize,
}

impl<T: Clone> WorkerFarm<T> {
  pub fn from_rx(
    rx: Receiver<Sender<T>>,
    threads: usize,
  ) -> Self {
    let mut senders = vec![];
    let mut c = 0;

    while let Ok(sender) = rx.recv() {
      senders.push(sender);
      c += 1;
      if c == threads {
        break;
      }
    }

    Self { senders, c: 0 }
  }

  pub fn len(&self) -> usize {
    return self.senders.len();
  }

  pub fn send_all(
    &self,
    msg: T,
  ) {
    for sender in self.senders.iter() {
      sender.send(msg.clone()).unwrap();
    }
  }

  pub fn send(
    &mut self,
    msg: T,
  ) {
    self.senders[self.c].send(msg).unwrap();
    self.c += 1;
    if self.c > self.senders.len() - 1 {
      self.c = 0;
    }
  }
}