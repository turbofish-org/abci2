#![feature(test)]

extern crate test;

use abci2::messages::abci::*;
use protobuf::Message;

#[bench]
fn check_tx_req_decode(b: &mut test::Bencher) {
    let mut req = Request::new();
    let mut check_tx = RequestCheckTx::new();
    check_tx.set_tx(vec![123; 64]);
    req.set_check_tx(check_tx);

    let bytes = req.write_to_bytes().unwrap();

    b.iter(|| protobuf::parse_from_bytes::<Request>(bytes.as_slice()));
}

#[bench]
fn check_tx_res_encode(b: &mut test::Bencher) {
    let mut res = Response::new();
    let mut check_tx = ResponseCheckTx::new();
    check_tx.set_code(0);
    res.set_check_tx(check_tx);

    b.iter(|| res.write_to_bytes().unwrap());
}
