#![feature(mpsc_select)]

extern crate notify;
extern crate comm;

use std::sync::mpsc;
use std::thread;
use std::fs::File;
use std::time::Duration;

use notify::{RecommendedWatcher, Watcher};

use comm::spmc::unbounded::Consumer;
use comm::spmc;

fn sleep_til_death(rx: Consumer<bool>) {
    rx.recv_sync().unwrap();
    println!("thread exiting!");
}

fn main() {
    let mut count = 0;
    let (threadtx, threadrx) = spmc::unbounded::new();

    let (starttx, startrx) = mpsc::channel();
    let (stoptx, stoprx) = mpsc::channel();

    let mut sw: RecommendedWatcher = Watcher::new(starttx).unwrap();
    let mut stw: RecommendedWatcher = Watcher::new(stoptx).unwrap();

    File::create("/tmp/tstart").unwrap();
    File::create("/tmp/tstop").unwrap();

    println!("touch /tmp/tstart to start threads");
    println!("touch /tmp/tstop to stop threads");

    // sleep for a bit so we don't catch the create in the watch
    thread::sleep(Duration::new(1, 0));

    sw.watch("/tmp/tstart").unwrap();
    stw.watch("/tmp/tstop").unwrap();

    println!("waiting!");

    loop {
        select! {
            _ = startrx.recv() => {
                count = count+1;
                println!("starting a thread! current count: {}", count);
                let t = threadrx.clone();
                thread::spawn(|| {
                    sleep_til_death(t);
                });
            },
            _ = stoprx.recv() => {
                count = count-1;
                println!("stopping a thread! count is now {}", count);
                threadtx.send(true).unwrap();
            }
        }
    }
}
