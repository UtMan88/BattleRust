use std::thread;
use std::net::TcpStream;

pub struct MessageSystem {
    stream: Option<TcpStream>,
}

impl MessageSystem {

    pub fn new(msgListener: TcpStream) -> Self {
        MessageSystem {
            stream: Some(msgListener),
        }
    }

    pub fn start<F: FnOnce(TcpStream) + Send + 'static>(&mut self, f: F) {
        let l: TcpStream = self.stream.take().unwrap().try_clone().unwrap();

        thread::spawn(move || { f(l); });
    }

    pub fn with_task<F: FnOnce(TcpStream) + Send + 'static>(mut self, task: F)
                                                          -> Self {
        self.start(task);
        self
    }
}