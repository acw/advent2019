use std::sync::mpsc;

pub struct Sender<T> {
    underlying: mpsc::Sender<Option<T>>,
    done: bool
}

impl<T> Sender<T> {
    pub fn send(&self, t: T) {
        if self.done {
            panic!("Sending on previously-closed channel.");
        }
        self.underlying.send(Some(t)).expect("Channel mysteriously closed on send.");
    }

    pub fn conclude(&mut self) {
        self.underlying.send(None).expect("Failed to close channel.");
        self.done = true;
    }
}

pub struct Receiver<T> {
    underlying: mpsc::Receiver<Option<T>>,
    done: bool
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if self.done {
            return None;
        }

        match self.underlying.recv() {
            Err(e) => panic!("Channel mysteriously closed on recv: {}", e),
            Ok(None) => { self.done = true; None },
            Ok(Some(x)) => Some(x),
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (undersend, underrecv) = mpsc::channel();
    let sender   = Sender{   underlying: undersend, done: false };
    let receiver = Receiver{ underlying: underrecv, done: false };
    (sender, receiver)
}
