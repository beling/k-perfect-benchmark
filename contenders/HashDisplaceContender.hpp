#pragma once

#include <HashDisplace.hpp>
#include "Contender.h"

template<size_t k, typename BucketFunction, typename Encoding>
class HashDisplaceContender : public Contender {
    public:
        kphf::HashDisplace::HashDisplace<k, BucketFunction, Encoding> kphf;
        size_t bucketSize;

        HashDisplaceContender(const size_t N, const size_t bucketSize)
                : Contender(N, k, 1.0), bucketSize(bucketSize) {
        }

        std::string name() override {
            return std::string("HashDisplace")
                    + " bucketSize=" + std::to_string(bucketSize)
                    + " bucketFunction=" + BucketFunction::name()
                    + " encoding=" + Encoding::name();
        }

        void construct() override {
            kphf = kphf::HashDisplace::HashDisplace<k, BucketFunction, Encoding>(keys, bucketSize);
        }

        size_t sizeBits() override {
            return kphf.count_bits();
        }

        void performQueries() override {
            auto x = [&] (std::string &key) {
                return kphf(key);
            };
            doPerformQueries(keys, x);
        }

        size_t keyValue(size_t key_index) override {
            return kphf(keys[key_index]);
        }
};

void hashDisplaceContenderRunner(size_t N, size_t k);
