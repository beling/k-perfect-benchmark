#pragma once

#include <ThresholdBasedBumping.hpp>
#include "Contender.h"

template <size_t k, int thresholdSize = bytehamster::util::ceillog2(k), bool packing = true>
class ThresholdBasedBumpingContender : public Contender {
    public:
        using ThresholdBasedBumping = kphf::ThresholdBasedBumping::ThresholdBasedBumping<k, thresholdSize, packing>;
        ThresholdBasedBumping kphf;
        double overload;

        explicit ThresholdBasedBumpingContender(size_t N, double overload)
                : Contender(N, k, 1.0), overload(overload) {
        }

        std::string name() override {
            return std::string("ThresholdBasedBumping")
                    + " overload=" + std::to_string(overload)
                    + " thresholdSize=" + std::to_string(thresholdSize)
                    + " packing=" + std::to_string(packing);
        }

        void construct() override {
            kphf = ThresholdBasedBumping(keys, overload);
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

void thresholdBasedBumpingContenderRunner(size_t N, size_t k);
