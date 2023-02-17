use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, Thread},
};

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    //定义存储线程的类型
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

// struct Job;

type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

//实现FnBox的类型，可以在Box上调用方法
trait FnBox {
    fn call_box(self: Box<Self>);
}

//所有实现了FnOnce的类型都实现了call_box方法

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // let thread = thread::spawn(||{
        //     receiver
        // });
        let thread = thread::spawn(move || loop {
            // let job = receiver.lock().unwrap().recv().unwrap();
            // //锁定任务然后接受任务
            // println!("Worker {} got a job ;Executing.......",id);
            // // (*job)();
            // job.call_box();
            // // job();

            //使用while使其他线程无法正常接受任务
            // while let Ok(job) = receiver.lock().unwrap().recv(){
            //     println!("Worker {} got a job ;Executing.......",id);
            //     job.call_box();
            // }

            //使用enum Message
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job ;Executing.......", id);
                    job.call_box();
                }
                Message::Terminate=>{
                    println!("Worker {}is telled to terminate", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    /// create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// the 'new' function will panic if the size is zero

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        //创建预分配好的Vector
        let mut workers = Vec::with_capacity(size);

        //管道：多个发送者和一个接受者
        //获取发送者和接受者
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            //关于spawn函数
            /*
            pub fn spawn<F, T>(f: F) -> JoinHandle<T>
            where
                F: FnOnce() -> T,
                F: Send + 'static,
                T: Send + 'static,
            {
                Builder::new().spawn(f).expect("failed to spawn thread")
            }
            */

            //创建线程存储到threads中
            //线程创建之后进入等待状态，而非和spawn函数一样，是直接执行的

            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        println!("Sending terminate message to all workers.");

        //发送终止执行的消息
        for _ in &mut self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }


        for worker in &mut self.workers {

 
            println!("shutting down worker {}", worker.id);

            // // worker.thread.join().unwrap();

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }


            
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn test_fnOnce() {
        assert_eq!(1, 1);
        let s = Arc::new(String::from("多线程漫游"));
        for _ in 0..10 {
            let s = Arc::clone(&s);
            let handle = thread::spawn(move || println!("{}", s));
        }
    }
}
