#[cfg(test)]
mod benchs {

    extern crate test;
    use test::Bencher;
    use super::super::launch;

    #[bench]
    fn bench_main(b: &mut Bencher) {
        b.iter(|| launch(100));
    }
}
