use ph::{GetSize, phast::{self, Conf, DefaultCompressedArray, GenericCore, SeedOnlyK, SeedOnlyKCore, compressed_array::CompactFast}};
use std::hint::black_box;

pub enum PHastVariant {
    //BitsEF(phast::KFunction<GenericCore, ph::seeds::BitsFast, SeedOnlyKCore, DefaultCompressedArray>),
    Bits8EF(phast::KFunction<GenericCore, ph::seeds::Bits8, SeedOnlyKCore, DefaultCompressedArray>),
    //Bits4EF(phast::KFunction<GenericCore, ph::seeds::TwoToPowerBitsStatic::<2>, SeedOnlyKCore, DefaultCompressedArray>),
    //BitsC(phast::KFunction<GenericCore, ph::seeds::BitsFast, SeedOnlyKCore, CompactFast>),
    Bits8C(phast::KFunction<GenericCore, ph::seeds::Bits8, SeedOnlyKCore, CompactFast>),
    //Bits4C(phast::KFunction<GenericCore, ph::seeds::TwoToPowerBitsStatic::<2>, SeedOnlyKCore, CompactFast>),
    None
}

#[no_mangle]
pub extern "C" fn createPhastStruct() -> *mut PHastVariant {
    Box::into_raw(Box::new(PHastVariant::None))
}

#[no_mangle]
pub extern "C" fn constructPhast(struct_ptr: *mut PHastVariant, keys_ptr: *const Box<[Box<[u8]>]>,
                                 k: u16, bits_per_seed: u8, bucket_size100: u16, threads_num: usize, ef: bool) {
    let f = unsafe { &mut *struct_ptr };
    let keys = unsafe { &*keys_ptr };
    *f = match (bits_per_seed, ef) {
        (8, true) => PHastVariant::Bits8EF(phast::KFunction::with_slice_conf_threads_sc(
            &keys[..],
            Conf::generic8(bucket_size100),
            threads_num,
            SeedOnlyK::new(k)
        )),
        (8, false) => PHastVariant::Bits8C(phast::KFunction::with_slice_conf_threads_sc(
            &keys[..],
            Conf::generic8(bucket_size100),
            threads_num,
            SeedOnlyK::new(k)
        )),
        (_, _) => panic!("kPHast requires bits per seed = 8"),
    }
}

#[no_mangle]
pub extern "C" fn queryPhast(struct_ptr: *const PHastVariant, keys_ptr: *const Box<[Box<[u8]>]>, index: usize) -> u64 {
    let keys = &unsafe { &*keys_ptr }[..];
    let key = keys[index].as_ref();
    match unsafe { &*struct_ptr } {
        //PHastVariant::BitsEF(function) => function.get(key) as u64,
        PHastVariant::Bits8EF(function) => function.get(key) as u64,
        //PHastVariant::Bits4EF(function) => function.get(key) as u64,
        //PHastVariant::BitsC(function) => function.get(key) as u64,
        PHastVariant::Bits8C(function) => function.get(key) as u64,
        //PHastVariant::Bits4C(function) => function.get(key) as u64,
        PHastVariant::None => panic!("kPHast not constructed yet"),
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
        //PHastVariant::BitsEF(function) => //query_all(function, &keys),
        //    for key in keys { black_box(function.get(key.as_ref())); },
        PHastVariant::Bits8EF(function) =>  //query_all(function, &keys),
            for key in keys { black_box(function.get(key.as_ref())); },
        //PHastVariant::Bits4EF(function) =>  //query_all(function, &keys),
        //    for key in keys { black_box(function.get(key.as_ref())); },
        //PHastVariant::BitsC(function) => //query_all(function, &keys),
        //    for key in keys { black_box(function.get(key.as_ref())); },
        PHastVariant::Bits8C(function) => //query_all(function, &keys),
            for key in keys { black_box(function.get(key.as_ref())); },
        //PHastVariant::Bits4C(function) => //query_all(function, &keys),
        //    for key in keys { black_box(function.get(key.as_ref())); },
        PHastVariant::None => panic!("PHast not constructed yet"),
    }
}

#[no_mangle]
pub extern "C" fn sizePhast(struct_ptr: *const PHastVariant) -> usize {
    match unsafe { &*struct_ptr } {
        //PHastVariant::BitsEF(function) => function.size_bytes(),
        PHastVariant::Bits8EF(function) => function.size_bytes(),
        //PHastVariant::Bits4EF(function) => function.size_bytes(),
        //PHastVariant::BitsC(function) => function.size_bytes(),
        PHastVariant::Bits8C(function) => function.size_bytes(),
        //PHastVariant::Bits4C(function) => function.size_bytes(),
        PHastVariant::None => panic!("PHast not constructed yet"),
    }
}

#[no_mangle]
pub extern "C" fn destroyPhastStruct(struct_instance: *mut PHastVariant) {
    unsafe { let _ = Box::from_raw(struct_instance); }
}
