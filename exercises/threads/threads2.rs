use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 使用 Arc 和 Mutex 来包装共享的状态
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250)); // 模拟工作
            println!("job started");

            // 锁定 Mutex 后更新 jobs_completed
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
    let status = status.lock().unwrap();

    // 输出最终的 jobs_completed 数量
    println!("Total jobs completed: {}", status.jobs_completed);
}
