use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use lazy_static::lazy_static;
use metrics_util::AtomicBucket;

lazy_static! {
    static ref RANDOM_INTS: Vec<u64> = vec![
        21061184, 21301862, 21331592, 21457012, 21500016, 21537837, 21581557, 21620030, 21664102,
        21678463, 21708437, 21751808, 21845243, 21850265, 21938879, 21971014, 22005842, 22034601,
        22085552, 22101746, 22115429, 22139883, 22260209, 22270768, 22298080, 22299780, 22307659,
        22354697, 22355668, 22359397, 22463872, 22496590, 22590978, 22603740, 22706352, 22820895,
        22849491, 22891538, 22912955, 22919915, 22928920, 22968656, 22985992, 23033739, 23061395,
        23077554, 23138588, 23185172, 23282479, 23290830, 23316844, 23386911, 23641319, 23677058,
        23742930, 25350389, 25399746, 25404925, 25464391, 25478415, 25480015, 25632783, 25639769,
        25645612, 25688228, 25724427, 25862192, 25954476, 25994479, 26008752, 26036460, 26038202,
        26078874, 26118327, 26132679, 26207601, 26262418, 26270737, 26274860, 26431248, 26434268,
        26562736, 26580134, 26593740, 26618561, 26844181, 26866971, 26907883, 27005270, 27023584,
        27024044, 27057184, 23061395, 23077554, 23138588, 23185172, 23282479, 23290830, 23316844,
        23386911, 23641319, 23677058, 23742930, 25350389, 25399746, 25404925, 25464391, 25478415,
        25480015, 25632783, 25639769, 25645612, 25688228, 25724427, 25862192, 25954476, 25994479,
        26008752, 26036460, 26038202, 26078874, 26118327, 26132679, 26207601, 26262418, 26270737,
        26274860, 26431248, 26434268, 26562736, 26580134, 26593740, 26618561, 26844181, 26866971,
        26907883, 27005270, 27023584, 27024044, 27057184, 23061395, 23077554, 23138588, 23185172,
        23282479, 23290830, 23316844, 23386911, 23641319, 23677058, 23742930, 25350389, 25399746,
        25404925, 25464391, 25478415, 25480015, 25632783, 25639769, 25645612, 25688228, 25724427,
        25862192, 25954476, 25994479, 26008752, 26036460, 26038202, 26078874, 26118327, 26132679,
        26207601, 26262418, 26270737, 26274860, 26431248, 26434268, 26562736, 26580134, 26593740,
        26618561, 26844181, 26866971, 26907883, 27005270, 27023584, 27024044, 27057184, 23061395,
        23077554, 23138588, 23185172, 23282479, 23290830, 23316844, 23386911, 23641319, 23677058,
        23742930, 25350389, 25399746, 25404925, 25464391, 25478415, 25480015, 25632783, 25639769,
        25645612, 25688228, 25724427, 25862192, 25954476, 25994479, 26008752, 26036460, 26038202,
        26078874, 26118327, 26132679, 26207601, 26262418, 26270737, 26274860, 26431248, 26434268,
        26562736, 26580134, 26593740, 26618561, 26844181, 26866971, 26907883, 27005270, 27023584,
        27024044, 27057184, 27088034, 27088550, 27302898, 27353925, 27412984, 27488633, 27514155,
        27558052, 27601937, 27606339, 27624514, 27680396, 27684064, 27963602, 27414982, 28450673
    ];
}

fn bucket_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("bucket");
    group.throughput(Throughput::Elements(RANDOM_INTS.len() as u64));
    group.bench_function("write", |b| {
        let bucket = AtomicBucket::new();

        b.iter(|| {
            for value in RANDOM_INTS.iter() {
                bucket.push(value);
            }
        })
    });
    group.finish();
}

criterion_group!(benches, bucket_benchmark);
criterion_main!(benches);