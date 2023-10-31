use {
    coolor::*,
    glassbench::*,
    rand::prelude::*,
};

fn bench_rgb_to_ansi(gb: &mut Bench) {
    gb.task("1000 rgb to ansi", |b| {
        let mut rng = StdRng::seed_from_u64(0);
        let rgbs: Vec<Rgb> = (0..1_000)
            .map(|_| Rgb::new(rng.gen(), rng.gen(), rng.gen()))
            .collect();
        b.iter(|| {
            for rgb in &rgbs {
                let ansi = rgb.to_ansi();
                pretend_used(ansi);
            }
        });
    });
    gb.task("ansi to rgb to ansi round trip", |b| {
        b.iter(|| {
            for code in 16..=255 {
                let c1 = AnsiColor { code };
                let c2 = c1.to_rgb();
                let c3 = c2.to_ansi();
                assert_eq!(c1, c3);
            }
        });
    });
}

glassbench!(
    "RGB to ANSI",
    bench_rgb_to_ansi,
);
