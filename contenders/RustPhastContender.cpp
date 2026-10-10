#include "RustPhastContender.hpp"

void rustPHastContenderRunnerWithEf(size_t N, const size_t k, bool use_ef) {
    for (size_t bucket_size_100 = 250; bucket_size_100 <= 320; bucket_size_100 += 10)
        RustPhastContender(N, k, 6, bucket_size_100, use_ef).run();

    for (size_t bucket_size_100 = 300; bucket_size_100 <= 380; bucket_size_100 += 10)
        RustPhastContender(N, k, 7, bucket_size_100, use_ef).run();

    for (size_t bucket_size_100 = 350; bucket_size_100 <= 450; bucket_size_100 += 10)
        RustPhastContender(N, k, 8, bucket_size_100, use_ef).run();

    for (size_t bucket_size_100 = 420; bucket_size_100 <= 520; bucket_size_100 += 10)
        RustPhastContender(N, k, 9, bucket_size_100, use_ef).run();
}

void rustPHastContenderRunner(size_t N, const size_t k) {
    rustPHastContenderRunnerWithEf(N, k, true);
    rustPHastContenderRunnerWithEf(N, k, false);
}