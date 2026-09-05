use ph::{GetSize, phast::{self, Conf, ConfTurbo, DefaultCompressedArray, Params, ParamsTurbo, SeedOnly, compressed_array::CompactFast}};
use std::hint::black_box;

pub enum PHastVariant {
    BitsEF(phast::Function<Conf, ph::seeds::BitsFast, SeedOnly, DefaultCompressedArray>),
    Bits8EF(phast::Function<Conf, ph::seeds::Bits8, SeedOnly, DefaultCompressedArray>),
    Bits4EF(phast::Function<Conf, ph::seeds::TwoToPowerBitsStatic::<2>, SeedOnly, DefaultCompressedArray>),
    BitsC(phast::Function<Conf, ph::seeds::BitsFast, SeedOnly, CompactFast>),
    Bits8C(phast::Function<Conf, ph::seeds::Bits8, SeedOnly, CompactFast>),
    Bits4C(phast::Function<Conf, ph::seeds::TwoToPowerBitsStatic::<2>, SeedOnly, CompactFast>),
    TurboEF(phast::Function<ConfTurbo, ph::seeds::Bits8, SeedOnly, DefaultCompressedArray>),
    TurboC(phast::Function<ConfTurbo, ph::seeds::Bits8, SeedOnly, CompactFast>),
    None
}

#[no_mangle]
pub extern "C" fn createPhastStruct() -> *mut PHastVariant {
    Box::into_raw(Box::new(PHastVariant::None))
}

#[no_mangle]
pub extern "C" fn constructPhast(struct_ptr: *mut PHastVariant, keys_ptr: *const Box<[Box<[u8]>]>,
                                 bits_per_seed: u8, bucket_size100: u16, threads_num: usize, ef: bool) {
    let f = unsafe { &mut *struct_ptr };
    let keys = unsafe { &*keys_ptr };
    *f = match (bits_per_seed, ef, bucket_size100) {
        (8, true, 0) => PHastVariant::TurboEF(phast::Function::with_slice_p_threads_hash_sc(
            &keys[..],
            &ParamsTurbo::new(ph::seeds::Bits8),
            threads_num,
            seedable_hash::BuildDefaultSeededHasher::default(),
            SeedOnly
        )),
        (8, false, 0) => PHastVariant::TurboC(phast::Function::with_slice_p_threads_hash_sc(
            &keys[..],
            &ParamsTurbo::new(ph::seeds::Bits8),
            threads_num,
            seedable_hash::BuildDefaultSeededHasher::default(),
            SeedOnly
        )),
        (_, _, 0) => panic!("PHast Turbo (bucket_size100=0) requires bits per seed = 8"),
        (8, true, _) => PHastVariant::Bits8EF(phast::Function::with_slice_p_threads_hash_sc(
            &keys[..],
            &Params::new(ph::seeds::Bits8, bucket_size100),
            threads_num,
            seedable_hash::BuildDefaultSeededHasher::default(),
            SeedOnly
        )),
        (4, true, _) => PHastVariant::Bits4EF(phast::Function::with_slice_p_threads_hash_sc(
            &keys[..],
            &Params::new(ph::seeds::TwoToPowerBitsStatic::<2>, bucket_size100),
            threads_num,
            seedable_hash::BuildDefaultSeededHasher::default(),
            SeedOnly
        )),
        (_, true, _) => PHastVariant::BitsEF(phast::Function::with_slice_p_threads_hash_sc(
            &keys[..],
            &Params::new(ph::seeds::BitsFast(bits_per_seed), bucket_size100),
            threads_num,
            seedable_hash::BuildDefaultSeededHasher::default(),
            SeedOnly
        )),

        (8, false, _) => PHastVariant::Bits8C(phast::Function::with_slice_p_threads_hash_sc(
            &keys[..],
            &Params::new(ph::seeds::Bits8, bucket_size100),
            threads_num,
            seedable_hash::BuildDefaultSeededHasher::default(),
            SeedOnly
        )),
        (4, false, _) => PHastVariant::Bits4C(phast::Function::with_slice_p_threads_hash_sc(
            &keys[..],
            &phast::Params::new(ph::seeds::TwoToPowerBitsStatic::<2>, bucket_size100),
            threads_num,
            seedable_hash::BuildDefaultSeededHasher::default(),
            SeedOnly
        )),
        (_, false, _) => PHastVariant::BitsC(phast::Function::with_slice_p_threads_hash_sc(
            &keys[..],
            &phast::Params::new(ph::seeds::BitsFast(bits_per_seed), bucket_size100),
            threads_num,
            seedable_hash::BuildDefaultSeededHasher::default(),
            SeedOnly
        )),
    }
}

#[no_mangle]
pub extern "C" fn queryPhast(struct_ptr: *const PHastVariant, keys_ptr: *const Box<[Box<[u8]>]>, index: usize) -> u64 {
    let keys = &unsafe { &*keys_ptr }[..];
    let key = keys[index].as_ref();
    match unsafe { &*struct_ptr } {
        PHastVariant::BitsEF(function) => function.get(key) as u64,
        PHastVariant::Bits8EF(function) => function.get(key) as u64,
        PHastVariant::Bits4EF(function) => function.get(key) as u64,
        PHastVariant::BitsC(function) => function.get(key) as u64,
        PHastVariant::Bits8C(function) => function.get(key) as u64,
        PHastVariant::Bits4C(function) => function.get(key) as u64,
        PHastVariant::TurboEF(function) => function.get(key) as u64,
        PHastVariant::TurboC(function) => function.get(key) as u64,
        PHastVariant::None => panic!("PHast not constructed yet"),
    }
}

/*#[inline(always)] fn query_all<SS, CA, S>(function: &phast::Function<SS, CA, S>, keys: &[Box<[u8]>])
    where SS: ph::fmph::SeedSize, CA: phast::CompressedArray, S: ph::BuildSeededHasher 
{
    for key in keys {
        black_box(function.get(key.as_ref()));
    }
}*/

#[no_mangle]
pub extern "C" fn queryPhastAll(struct_ptr: *const PHastVariant, keys_ptr: *const Box<[Box<[u8]>]>) {
    let keys = unsafe { &*keys_ptr };
    match unsafe { &*struct_ptr } {
        PHastVariant::BitsEF(function) => //query_all(function, &keys),
            for key in keys { black_box(function.get(key.as_ref())); },
        PHastVariant::Bits8EF(function) =>  //query_all(function, &keys),
            for key in keys { black_box(function.get(key.as_ref())); },
        PHastVariant::Bits4EF(function) =>  //query_all(function, &keys),
            for key in keys { black_box(function.get(key.as_ref())); },
        PHastVariant::BitsC(function) => //query_all(function, &keys),
            for key in keys { black_box(function.get(key.as_ref())); },
        PHastVariant::Bits8C(function) => //query_all(function, &keys),
            for key in keys { black_box(function.get(key.as_ref())); },
        PHastVariant::Bits4C(function) => //query_all(function, &keys),
            for key in keys { black_box(function.get(key.as_ref())); },
        PHastVariant::TurboEF(function) =>
            for key in keys { black_box(function.get(key.as_ref())); },
        PHastVariant::TurboC(function) =>
            for key in keys { black_box(function.get(key.as_ref())); },
        PHastVariant::None => panic!("PHast not constructed yet"),
    }
}

#[no_mangle]
pub extern "C" fn sizePhast(struct_ptr: *const PHastVariant) -> usize {
    match unsafe { &*struct_ptr } {
        PHastVariant::BitsEF(function) => function.size_bytes(),
        PHastVariant::Bits8EF(function) => function.size_bytes(),
        PHastVariant::Bits4EF(function) => function.size_bytes(),
        PHastVariant::BitsC(function) => function.size_bytes(),
        PHastVariant::Bits8C(function) => function.size_bytes(),
        PHastVariant::Bits4C(function) => function.size_bytes(),
        PHastVariant::TurboEF(function) => function.size_bytes(),
        PHastVariant::TurboC(function) => function.size_bytes(),
        PHastVariant::None => panic!("PHast not constructed yet"),
    }
}

#[no_mangle]
pub extern "C" fn destroyPhastStruct(struct_instance: *mut PHastVariant) {
    unsafe { let _ = Box::from_raw(struct_instance); }
}
