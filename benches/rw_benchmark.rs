use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use rw_builder::{RwBuilder, RwBuilderExt, VecBuilder};
use std::hint::black_box;
use std::io::Write;

fn bench_rw_builder(c: &mut Criterion) {
    let mut group = c.benchmark_group("Streaming Overhead");
    let data = vec![0u8; 1024 * 1024]; // 1MB payload

    group.throughput(Throughput::Bytes(data.len() as u64));

    // 1. Raw Vec Write
    group.bench_function("raw_vec_write", |b| {
        b.iter(|| {
            let mut writer = Vec::with_capacity(1024 * 1024);
            writer.write_all(black_box(&data)).unwrap();
            black_box(writer);
        });
    });

    // 2. rw-builder Vec Write
    group.bench_function("rw_builder_vec_write", |b| {
        b.iter(|| {
            let builder = VecBuilder::default();
            let mut writer = builder.writer().unwrap();
            writer.write_all(black_box(&data)).unwrap();
            black_box(writer);
        });
    });

    #[cfg(feature = "flate2")]
    {
        use flate2::write::DeflateEncoder;
        use flate2::Compression;

        // 3. Raw Deflate Write
        group.bench_function("raw_deflate_write", |b| {
            b.iter(|| {
                let vec = Vec::with_capacity(1024 * 1024);
                let mut encoder = DeflateEncoder::new(vec, Compression::fast());
                encoder.write_all(black_box(&data)).unwrap();
                let finished = encoder.finish().unwrap();
                black_box(finished);
            });
        });

        // 4. rw-builder Deflate Write
        group.bench_function("rw_builder_deflate_write", |b| {
            b.iter(|| {
                let builder = VecBuilder::default().deflate(Compression::fast());
                let mut writer = builder.writer().unwrap();
                writer.write_all(black_box(&data)).unwrap();
                writer.flush().unwrap();
                black_box(writer);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_rw_builder);
criterion_main!(benches);
