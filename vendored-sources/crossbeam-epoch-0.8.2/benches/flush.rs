#![feature(test)]

extern crate crossbeam_epoch as epoch;
extern crate crossbeam_utils as utils;
extern crate test;

use std::sync::Barrier;

use test::Bencher;
use utils::thread::scope;

#[bench]
fn single_flush(b: &mut Bencher) {
    const THREADS: usize = 16;

    let start = Barrier::new(THREADS + 1);
    let end = Barrier::new(THREADS + 1);

    scope(|s| {
        for _ in 0..THREADS {
            s.spawn(|_| {
                epoch::pin();
                start.wait();
                end.wait();
            });
        }

        start.wait();
        b.iter(|| epoch::pin().flush());
        end.wait();
    })
    .unwrap();
}

#[bench]
fn multi_flush(b: &mut Bencher) {
    const THREADS: usize = 16;
    const STEPS: usize = 10_000;

    b.iter(|| {
        scope(|s| {
            for _ in 0..THREADS {
                s.spawn(|_| {
                    for _ in 0..STEPS {
                        let guard = &epoch::pin();
                        guard.flush();
                    }
                });
            }
        })
        .unwrap();
    });
}
