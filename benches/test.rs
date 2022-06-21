#[macro_use]
extern crate bencher;

use bencher::Bencher;
use libts::{SysClock, TimeVal};

fn timeval_date_1k(bench: &mut Bencher) {
    let ts1 = TimeVal::from_ymd(2022, 1, 1).and_hms(8, 30, 0);
    bench.iter(|| {
        for _i in 0..1000 {
            (_, _, _) = ts1.date();
        }
    })
}

fn sysclock_1k(bench: &mut Bencher) {
    let mut clk = SysClock::new(true);
    let ts1 = TimeVal::from_ymd(2022, 1, 1).and_hms(8, 30, 0);
    clk.set_timeval(&ts1);
    bench.iter(|| {
        for _i in 0..1000 {
            _ = clk.now();
        }
    })
}

benchmark_group!(benches, timeval_date_1k, sysclock_1k);
benchmark_main!(benches);
