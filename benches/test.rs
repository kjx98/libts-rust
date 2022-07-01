#[macro_use]
extern crate bencher;

//use serde::de::{self, DeserializeSeed, SeqAccess, Visitor};
use bencher::Bencher;
use libts::{from_bytes, from_msg, ClMessage, SysClock, UnixTime};
use serde::Deserialize;

fn timeval_date_1k(bench: &mut Bencher) {
    let ts1 = UnixTime::from_ymd(2022, 1, 1).and_hms(8, 30, 0);
    bench.iter(|| {
        for _i in 0..1000 {
            (_, _, _) = ts1.date();
        }
    })
}

fn sysclock_1k(bench: &mut Bencher) {
    let mut clk = SysClock::new(true);
    let ts1 = UnixTime::from_ymd(2022, 1, 1)
        .and_hms(8, 30, 0)
        .and_millis(0);
    clk.set_timeval(&ts1);
    bench.iter(|| {
        for _i in 0..1000 {
            _ = clk.now();
        }
    })
}

fn from_msg_struct(bench: &mut Bencher) {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Test<'a> {
        b: bool,
        int: u32,
        seq: Vec<String>,
        bb: &'a [u8],
    }

    let j = [
        0, 1u8, 0, 0, 0, 2u8, 1, b'a', 1, b'b', 4, b't', b'e', b's', b't',
    ];
    let msg = ClMessage::new(&j[..]);
    bench.iter(|| {
        _ = from_msg::<Test>(&msg).unwrap();
    })
}

fn from_msg_struct1(bench: &mut Bencher) {
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Test<'a> {
        b: bool,
        int: u32,
        bb: &'a [u8],
    }

    let j = [0, 1u8, 0, 0, 0, 4, b't', b'e', b's', b't'];
    let msg = ClMessage::new(&j[..]);
    bench.iter(|| {
        _ = from_msg::<Test>(&msg).unwrap();
    })
}

fn from_bytes_struct(bench: &mut Bencher) {
    let j = [0, 1u8, 0, 0, 0, 4, b't', b'e', b's', b't'];
    bench.iter(|| {
        let _b: bool = from_bytes(&j[0..1]).unwrap();
        let _int: u32 = from_bytes(&j[1..5]).unwrap();
        let _bb: &[u8] = from_bytes(&j[5..]).unwrap();
    })
}

benchmark_group!(
    benches,
    timeval_date_1k,
    sysclock_1k,
    from_msg_struct,
    from_msg_struct1,
    from_bytes_struct,
);
benchmark_main!(benches);
