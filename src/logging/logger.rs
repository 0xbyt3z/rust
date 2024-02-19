use std::fs;
use tracing_subscriber;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_appender;
use tracing_appender::non_blocking;

pub fn init_subscriber() {

    // let appender = tracing_appender::rolling::daily("./logs/","log.txt");
    // let (non_blocking, _guard) = tracing_appender::non_blocking(appender);
    let subscriber = tracing_subscriber::fmt()
        // .with_writer(non_blocking)
       //shows the file path in the logs
       .with_file(true)
       //shows the thread id in the log lines
       .with_thread_ids(false)
       .finish();
    subscriber.init();
}