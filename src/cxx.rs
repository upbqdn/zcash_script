//! Rust bindings for Zcash transparent scripts.

#![allow(missing_docs)]
#![allow(clippy::needless_lifetimes)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(clippy::unwrap_or_default)]

// Use the generated C++ bindings
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Dummy definitions for no_std
#[cfg(not(feature = "std"))]
pub type ScriptError = i32;

#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_OK: ScriptError = 0;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_EVAL_FALSE: ScriptError = 1;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_UNKNOWN_ERROR: ScriptError = 2;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_OP_RETURN: ScriptError = 3;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_SCRIPT_SIZE: ScriptError = 4;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_PUSH_SIZE: ScriptError = 5;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_OP_COUNT: ScriptError = 6;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_STACK_SIZE: ScriptError = 7;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_SIG_COUNT: ScriptError = 8;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_PUBKEY_COUNT: ScriptError = 9;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_VERIFY: ScriptError = 10;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_EQUALVERIFY: ScriptError = 11;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_CHECKMULTISIGVERIFY: ScriptError = 12;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_CHECKSIGVERIFY: ScriptError = 13;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_NUMEQUALVERIFY: ScriptError = 14;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_BAD_OPCODE: ScriptError = 15;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_DISABLED_OPCODE: ScriptError = 16;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_INVALID_STACK_OPERATION: ScriptError = 17;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_INVALID_ALTSTACK_OPERATION: ScriptError = 18;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_UNBALANCED_CONDITIONAL: ScriptError = 19;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_NEGATIVE_LOCKTIME: ScriptError = 20;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_UNSATISFIED_LOCKTIME: ScriptError = 21;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_SIG_HASHTYPE: ScriptError = 22;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_SIG_DER: ScriptError = 23;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_MINIMALDATA: ScriptError = 24;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_SIG_PUSHONLY: ScriptError = 25;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_SIG_HIGH_S: ScriptError = 26;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_SIG_NULLDUMMY: ScriptError = 27;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_PUBKEYTYPE: ScriptError = 28;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_CLEANSTACK: ScriptError = 29;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_DISCOURAGE_UPGRADABLE_NOPS: ScriptError = 30;
#[cfg(not(feature = "std"))]
pub const ScriptError_t_SCRIPT_ERR_VERIFY_SCRIPT: ScriptError = 31;

// Dummy functions for no_std
#[cfg(not(feature = "std"))]
pub unsafe fn zcash_script_verify_callback(
    _ctx: *const core::ffi::c_void,
    _cb: Option<extern "C" fn(*mut u8, u32, *const core::ffi::c_void, *const u8, u32, i32)>,
    _lock_time: i64,
    _is_final: i32,
    _script_pub_key: *const u8,
    _script_pub_key_len: u32,
    _script_sig: *const u8,
    _script_sig_len: u32,
    _flags: u32,
    _err: *mut ScriptError,
) -> i32 {
    0 // Always return false in no_std mode since we can't call C++
}

#[cfg(not(feature = "std"))]
pub unsafe fn zcash_script_legacy_sigop_count_script(
    _script_pub_key: *const u8,
    _script_pub_key_len: u32,
) -> u32 {
    0 // Return 0 in no_std mode
}
#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    use std::ffi::{c_int, c_uint, c_void};
    #[cfg(not(feature = "std"))]
    use core::ffi::{c_int, c_uint, c_void};

    use alloc::vec::Vec;
    use hex::FromHex;

    lazy_static::lazy_static! {
        pub static ref SCRIPT_PUBKEY: Vec<u8> = <Vec<u8>>::from_hex("a914c117756dcbe144a12a7c33a77cfa81aa5aeeb38187").unwrap();
        pub static ref SCRIPT_SIG: Vec<u8> = <Vec<u8>>::from_hex("00483045022100d2ab3e6258fe244fa442cfb38f6cef9ac9a18c54e70b2f508e83fa87e20d040502200eead947521de943831d07a350e45af8e36c2166984a8636f0a8811ff03ed09401473044022013e15d865010c257eef133064ef69a780b4bc7ebe6eda367504e806614f940c3022062fdbc8c2d049f91db2042d6c9771de6f1ef0b3b1fea76c1ab5542e44ed29ed8014c69522103b2cc71d23eb30020a4893982a1e2d352da0d20ee657fa02901c432758909ed8f21029d1e9a9354c0d2aee9ffd0f0cea6c39bbf98c4066cf143115ba2279d0ba7dabe2103e32096b63fd57f3308149d238dcbb24d8d28aad95c0e4e74e3e5e6a11b61bcc453ae").expect("Block bytes are in valid hex representation");
    }

    extern "C" fn sighash(
        sighash_out: *mut u8,
        sighash_out_len: c_uint,
        ctx: *const c_void,
        _script_code: *const u8,
        _script_code_len: c_uint,
        _hash_type: c_int,
    ) {
        unsafe {
            assert!(ctx.is_null());
            let sighash =
                hex::decode("e8c7bdac77f6bb1f3aba2eaa1fada551a9c8b3b5ecd1ef86e6e58a5f1aab952c")
                    .unwrap();
            assert!(sighash_out_len == sighash.len() as c_uint);
            std::ptr::copy_nonoverlapping(sighash.as_ptr(), sighash_out, sighash.len());
        }
    }

    extern "C" fn invalid_sighash(
        sighash_out: *mut u8,
        sighash_out_len: c_uint,
        ctx: *const c_void,
        _script_code: *const u8,
        _script_code_len: c_uint,
        _hash_type: c_int,
    ) {
        unsafe {
            assert!(ctx.is_null());
            let sighash =
                hex::decode("08c7bdac77f6bb1f3aba2eaa1fada551a9c8b3b5ecd1ef86e6e58a5f1aab952c")
                    .unwrap();
            assert!(sighash_out_len == sighash.len() as c_uint);
            std::ptr::copy_nonoverlapping(sighash.as_ptr(), sighash_out, sighash.len());
        }
    }

    #[test]
    fn it_works() {
        let nLockTime: i64 = 2410374;
        let isFinal: u8 = 1;
        let script_pub_key = &*SCRIPT_PUBKEY;
        let script_sig = &*SCRIPT_SIG;
        let flags: c_uint = 513;
        let mut err = 0;

        let ret = unsafe {
            super::zcash_script_verify_callback(
                std::ptr::null(),
                Some(sighash),
                nLockTime,
                isFinal,
                script_pub_key.as_ptr(),
                script_pub_key.len() as c_uint,
                script_sig.as_ptr(),
                script_sig.len() as c_uint,
                flags,
                &mut err,
            )
        };

        assert!(ret == 1);
    }

    #[test]
    fn it_fails_on_invalid_sighash() {
        let nLockTime: i64 = 2410374;
        let isFinal: u8 = 1;
        let script_pub_key = &*SCRIPT_PUBKEY;
        let script_sig = &*SCRIPT_SIG;
        let flags: c_uint = 513;
        let mut err = 0;

        let ret = unsafe {
            super::zcash_script_verify_callback(
                std::ptr::null(),
                Some(invalid_sighash),
                nLockTime,
                isFinal,
                script_pub_key.as_ptr(),
                script_pub_key.len() as c_uint,
                script_sig.as_ptr(),
                script_sig.len() as c_uint,
                flags,
                &mut err,
            )
        };

        assert!(ret != 1);
    }
}
