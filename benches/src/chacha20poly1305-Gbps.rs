use criterion::{
    criterion_group, criterion_main,
    measurement::{Measurement, ValueFormatter},
    BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};
use std::time::{Duration, Instant};

use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::ChaCha20Poly1305;


struct GigaBitsPerSecFormatter;
impl GigaBitsPerSecFormatter {
    fn giga_bits_per_second(&self, bytes: f64, _typical: f64, values: &mut [f64]) -> &'static str {
        let (denominator, unit) = (1000.0 * 1000.0 * 1000.0, "Gbits/s");

        for val in values {
            let bits_per_second = bytes * 8 as f64 * (1e9 / *val);
            *val = bits_per_second / denominator;
        }

        unit
    }

    fn elements_per_second(&self, elems: f64, typical: f64, values: &mut [f64]) -> &'static str {
        let elems_per_second = elems * (1e9 / typical);
        let (denominator, unit) = if elems_per_second < 1000.0 {
            (1.0, " elem/s")
        } else if elems_per_second < 1000.0 * 1000.0 {
            (1000.0, "Kelem/s")
        } else if elems_per_second < 1000.0 * 1000.0 * 1000.0 {
            (1000.0 * 1000.0, "Melem/s")
        } else {
            (1000.0 * 1000.0 * 1000.0, "Gelem/s")
        };

        for val in values {
            let elems_per_second = elems * (1e9 / *val);
            *val = elems_per_second / denominator;
        }

        unit
    }
}
impl ValueFormatter for GigaBitsPerSecFormatter {
    fn scale_throughputs(
        &self,
        typical: f64,
        throughput: &Throughput,
        values: &mut [f64],
    ) -> &'static str {
        match *throughput {
            Throughput::Bytes(bytes) => self.giga_bits_per_second(bytes as f64, typical, values),
            Throughput::Elements(elems) => self.elements_per_second(elems as f64, typical, values),
        }
    }

    fn scale_values(&self, ns: f64, values: &mut [f64]) -> &'static str {
        let (factor, unit) = if ns < 10f64.powi(0) {
            (10f64.powi(3), "ps")
        } else if ns < 10f64.powi(3) {
            (10f64.powi(0), "ns")
        } else if ns < 10f64.powi(6) {
            (10f64.powi(-3), "us")
        } else if ns < 10f64.powi(9) {
            (10f64.powi(-6), "ms")
        } else {
            (10f64.powi(-9), "s")
        };

        for val in values {
            *val *= factor;
        }

        unit
    }

    fn scale_for_machines(&self, _values: &mut [f64]) -> &'static str {
        // no scaling is needed
        "ns"
    }
}

struct GigaBitsPerSec;
impl Measurement for GigaBitsPerSec {
    type Intermediate = Instant;
    type Value = Duration;

    fn start(&self) -> Self::Intermediate {
        Instant::now()
    }
    fn end(&self, i: Self::Intermediate) -> Self::Value {
        i.elapsed()
    }
    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        *v1 + *v2
    }
    fn zero(&self) -> Self::Value {
        Duration::from_secs(0)
    }
    fn to_f64(&self, val: &Self::Value) -> f64 {
        val.as_nanos() as f64
    }
    fn formatter(&self) -> &dyn ValueFormatter {
        &GigaBitsPerSecFormatter
    }
}

fn bench_gbps(c: &mut Criterion<GigaBitsPerSec>) {
    let mut group = c.benchmark_group("chacha20poly1305-Gbps");

    bench_group(&mut group);
    group.finish();
}

fn bench_group<M: Measurement>(group: &mut BenchmarkGroup<M>) {
    const CUSTOM_SIZE: usize = 1420; // bytes
    for size in &[64, 512, 1024, CUSTOM_SIZE] {
        let buf = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(BenchmarkId::new("encrypt", size), |b| {
            let cipher = ChaCha20Poly1305::new(&Default::default());
            b.iter(|| cipher.encrypt(&Default::default(), &*buf))
        });
        group.bench_function(BenchmarkId::new("decrypt", size), |b| {
            let cipher = ChaCha20Poly1305::new(&Default::default());
            b.iter(|| cipher.decrypt(&Default::default(), &*buf))
        });
    }
}

criterion_group!(
    name = benches_gbps;
    config = Criterion::default().with_measurement(GigaBitsPerSec);
    targets = bench_gbps
);

criterion_main!(benches_gbps);
