#pragma once

#include <PaCHash.hpp>
#include "Contender.h"

class PaCHashContender : public Contender {
    public:
        kphf::PaCHash::PaCHash kphf;
        double a;

        PaCHashContender(const size_t N, const size_t k, const double a)
                : Contender(N, k, 1.0), a(a) {
        }

        std::string name() override {
            return std::string("PaCHash")
                    + " a=" + std::to_string(a);
        }

        void construct() override {
            kphf = kphf::PaCHash::PaCHash(k_contender, a, keys);
        }

        size_t sizeBits() override {
            return kphf.count_bits();
        }

        void performQueries() override {
            auto x = [&] (const std::string &key) {
                return kphf(key);
            };
            doPerformQueries(keys, x);
        }

        size_t keyValue(size_t key_index) override {
            return kphf(keys[key_index]);
        }
};

void paCHashContenderRunner(size_t N, size_t k);
