use crate::{
    mcd_bindings::{
        mcd_api_version_st, mcd_impl_version_info_st, MCD_API_VER_AUTHOR, MCD_API_VER_MAJOR,
        MCD_API_VER_MINOR,
    },
    MCD_LIB,
};

/// Initialize the library
pub fn init() {
    log::debug!("Initializing MCD library");
    let mut author = [0i8; 32];
    let string = MCD_API_VER_AUTHOR.map(|c| c as i8);
    author[0..(string.len())].copy_from_slice(string.as_slice());
    let version_requirement = mcd_api_version_st {
        v_api_major: MCD_API_VER_MAJOR as u16,
        v_api_minor: MCD_API_VER_MINOR as u16,
        author,
    };
    let mut output = mcd_impl_version_info_st {
        v_api: mcd_api_version_st {
            v_api_major: 0,
            v_api_minor: 0,
            author: [0; 32],
        },
        v_imp_major: 0,
        v_imp_minor: 0,
        v_imp_build: 0,
        vendor: [0; 32],
        date: [0; 16],
    };
    let result = unsafe { MCD_LIB.mcd_initialize_f(&version_requirement, &mut output) };
    assert_eq!(result, 0);
}
