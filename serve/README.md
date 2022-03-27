# Serve

> `Rust`版`serve`

## Functions

- Serve a directory with a single command!
- with gzip compress static files

## Thread pool

A thread pool is a **group of spawned threads**that are waiting and ready to handle a task. When the program receives a new task, it assigns one of the threads in the pool to the task, and that thread will process the task. The remaining threads in the pool are available to handle any other tasks that come in while the first thread is processing. When the first thread is done processing its task, it’s returned to the pool of idle threads, ready to handle a new task. A thread pool allows you to process connections concurrently, increasing the throughput of your server.

spawning unlimited threads => DDOS

> we’ll have a fixed number of threads waiting in the pool

## many ways to improve the throughput of a web server
### 1
As requests come in, they’ll be sent to the pool for processing. The pool will maintain a queue of incoming requests. Each of the threads in the pool will pop off a request from this queue, handle the request, and then ask the queue for another request. With this design, we can process **N** requests concurrently, where **N** is the number of threads. If each thread is responding to a long-running request, subsequent requests can still back up in the queue, but we’ve increased the number of long-running requests we can handle before reaching that point.

### 2
explore are the fork/join model and the single-threaded async I/O model. If you’re interested in this topic, you can read more about other solutions and try to implement them in Rust; with a low-level language like Rust, all of these options are possible.