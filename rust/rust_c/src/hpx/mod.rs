use alloc::string::{String, ToString};
use crate::common::types::{PtrString, PtrUR};
use crate::common::utils::convert_c_char;
use crate::{extract_ptr_with_type};
use ur_registry::hpx::app_call_device::HpxAppCallDevice;

#[no_mangle]
pub unsafe extern "C" fn hpx_app_call_get_payload(ptr: PtrUR) -> PtrString {
    let ur = extract_ptr_with_type!(ptr, HpxAppCallDevice);
    match String::from_utf8(ur.get_bytes()) {
        Ok(payload) => convert_c_char(payload),
        Err(_) => convert_c_char("".to_string()),
    }
}
