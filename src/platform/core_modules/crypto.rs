pub enum AesOperation {
    Encrypt,
    Decrypt,
}

#[derive(Debug)]
pub enum AesResult {
    Success,
    KeyNull,
    TaskBusy,
    InvalidKeyLength,
    UnkownResult,
}

pub fn aes_operation_sync(
    key: &[u8],
    in_data: &[u8],
    out_data: &mut [u8],
    op: AesOperation,
) -> AesResult {
    let key_len = key.len() as u8;
    let key = key.as_ptr() as *mut u8;
    let in_data_len = in_data.len() as u32;
    let in_data = in_data.as_ptr() as *mut u8;

    let result = unsafe {
        crate::bindings::aes_operation(
            key,
            key_len,
            in_data,
            in_data_len,
            out_data.as_mut_ptr(),
            out_data.len() as u32,
            match op {
                AesOperation::Encrypt => 1,
                AesOperation::Decrypt => 0,
            },
            None,
            0,
        )
    };

    match result {
        0 => AesResult::Success,
        -1 => AesResult::KeyNull,
        -2 => AesResult::TaskBusy,
        -4 => AesResult::InvalidKeyLength,
        _ => AesResult::UnkownResult,
    }
}
