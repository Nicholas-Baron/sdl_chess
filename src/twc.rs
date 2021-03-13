use std::sync::mpsc::{self, Receiver, Sender};

pub struct TwoWayChannel<S, R> {
    send: Sender<S>,
    recv: Receiver<R>,
}

pub fn establish<T1, T2>() -> (TwoWayChannel<T1, T2>, TwoWayChannel<T2, T1>) {
    let (send_t1, recv_t1) = mpsc::channel::<T1>();
    let (send_t2, recv_t2) = mpsc::channel::<T2>();
    (
        TwoWayChannel {
            send: send_t1,
            recv: recv_t2,
        },
        TwoWayChannel {
            send: send_t2,
            recv: recv_t1,
        },
    )
}

impl<S, R> TwoWayChannel<S, R> {
    pub fn recv(&self) -> Option<R> {
        self.recv.recv().ok()
    }

    pub fn send(&self, s: S) {
        self.send.send(s).unwrap();
    }

    pub fn try_recv(&self) -> TryRecvResult<R> {
        self.recv.try_recv().into()
    }
}

pub enum TryRecvResult<T> {
    Received(T),
    Empty,
    Disconnected,
}

impl<T> From<Result<T, mpsc::TryRecvError>> for TryRecvResult<T> {
    fn from(res: Result<T, mpsc::TryRecvError>) -> Self {
        use mpsc::TryRecvError::*;
        match res {
            Ok(t) => Self::Received(t),
            Err(Empty) => Self::Empty,
            Err(Disconnected) => Self::Disconnected,
        }
    }
}
