#include "RustPhastContender.hpp"

void rustPHastContenderRunnerWithEf(size_t N, const size_t k, bool use_ef) {
    for (size_t bucket_size_100 = 80; bucket_size_100 <= 170; bucket_size_100 += 5)
        RustPhastContender(N, k, 2, bucket_size_100, use_ef).run();

    for (size_t bucket_size_100 = 100; bucket_size_100 <= 190; bucket_size_100 += 5)
        RustPhastContender(N, k, 3, bucket_size_100, use_ef).run();

    for (size_t bucket_size_100 = 170; bucket_size_100 <= 240; bucket_size_100 += 5)
        RustPhastContender(N, k, 4, bucket_size_100, use_ef).run();

    for (size_t bucket_size_100 = 190; bucket_size_100 <= 260; bucket_size_100 += 5)
        RustPhastContender(N, k, 5, bucket_size_100, use_ef).run();

    for (size_t bucket_size_100 = 250; bucket_size_100 <= 320; bucket_size_100 += 5)
        RustPhastContender(N, k, 6, bucket_size_100, use_ef).run();

    for (size_t bucket_size_100 = 300; bucket_size_100 <= 380; bucket_size_100 += 5)
        RustPhastContender(N, k, 7, bucket_size_100, use_ef).run();

    for (size_t bucket_size_100 = 350; bucket_size_100 <= 450; bucket_size_100 += 5)
        RustPhastContender(N, k, 8, bucket_size_100, use_ef).run();

    for (size_t bucket_size_100 = 420; bucket_size_100 <= 520; bucket_size_100 += 5)
        RustPhastContender(N, k, 9, bucket_size_100, use_ef).run();
}

void rustPHastContenderRunner(size_t N, const size_t k) {
    rustPHastContenderRunnerWithEf(N, k, true);
    rustPHastContenderRunnerWithEf(N, k, false);
}