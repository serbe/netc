#![feature(test)]
use netc;
use test::Bencher;

extern crate test;

#[bench]
fn bench_bytes_insertion_and_remove(b: &mut Bencher) {
    let mut header = netc::header::Headers::new();
    let name = "Content-Type";
    let value = "text/html; charset=utf-8";
    b.iter(|| {
        assert_eq!(header.insert(name, value).unwrap(), None);
        assert_eq!(
            header.get(name),
            Some(&netc::header::HeaderFields::new(name, value).unwrap())
        );
        assert_eq!(
            header.remove(name),
            Some(netc::header::HeaderFields::new(name, value).unwrap())
        );
    });
}

#[bench]
fn bench_string_insertion_and_remove(b: &mut Bencher) {
    let mut headers = netc::headers::Headers::new();
    let name = "Content-Type";
    let value = "text/html; charset=utf-8";
    b.iter(|| {
        assert_eq!(headers.insert(name, value), None);
        assert_eq!(headers.get(name), Some(value.to_string()));
        assert_eq!(headers.remove(name), Some(value.to_string()));
    });
}
