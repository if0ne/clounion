use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

use uuid::Uuid;

use storage_client::client::StorageClient;
use storage_client::config::Config;

async fn one_hundred_big_file_processing() {
    let config = Config::try_from_file("../../ClientTest.toml").await;
    let client = StorageClient::new(config);

    let jobs = (0..16).map(|_| async {
        let file = tokio::fs::File::open("../../CargoBig.lock").await.unwrap();
        let user = Uuid::new_v4();
        let filename = format!("{}", user);
        client
            .create_large_file(&filename, file)
            .await
            .unwrap();

        client.read_large_file(&filename).await.unwrap();
        client.delete_file(&filename).await.unwrap();
    });
    futures::future::join_all(jobs).await;
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench group");
    group.significance_level(0.1).sample_size(10);

    group.bench_function("one_hundred_big_file_processing", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| one_hundred_big_file_processing());
    });
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
