// 仅在测试时使用的utils
#[cfg(test)]
use anchor_lang::prelude::*;
use anchor_lang::ZeroCopy;
#[cfg(test)]
use std::cell::RefCell;
#[cfg(test)]
use std::rc::Rc;

/// only for test
/// mock 一个 AccountInfo
pub fn mock_account_info<'a>(
    key: &'a Pubkey,
    owner: &'a Pubkey,
    is_signer: bool,
    is_writable: bool,
    lamports: u64,
    data_len: usize,
) -> (AccountInfo<'a>, Rc<RefCell<&'a mut u64>>, Rc<RefCell<&'a mut [u8]>>) {
    // 关键点：lamports 和 data 必须由测试持有所有权，以保证 &'static mut 引用有效期
    let lamports_ptr: *mut u64 = Box::into_raw(Box::new(lamports));
    let data_ptr: *mut [u8] = Box::into_raw(vec![0u8; data_len].into_boxed_slice());

    // 构造 AccountInfo 所需的底层引用
    let account_info = unsafe {
        AccountInfo::new(
            key,
            is_signer,
            is_writable,
            &mut *lamports_ptr,
            &mut *data_ptr,
            owner,
            false,
        )
    };

    // 重新暴露底层引用，方便测试代码直接访问
    let lamports_rc = Rc::new(RefCell::new(unsafe { &mut *lamports_ptr }));
    let data_rc = Rc::new(RefCell::new(unsafe { &mut *data_ptr }));

    (account_info, lamports_rc, data_rc)
}

/// only for test
pub fn mock_anchor_account_info<'a, 'b, T: ZeroCopy>(
    key: &'a Pubkey,
    owner: &'a Pubkey,
    is_signer: bool,
    is_writable: bool,
    lamports: u64,
    account: &'b T,
) -> (AccountInfo<'a>, Rc<RefCell<&'a mut u64>>, Rc<RefCell<&'a mut [u8]>>) {
    // 计算 data 长度：8 字节 discriminator + 序列化数据
    let mut buf = Vec::new();
    // 预留 discriminator + 内容
    buf.extend_from_slice(T::DISCRIMINATOR);
    buf.extend_from_slice(bytemuck::bytes_of(account));

    // 构造 AccountInfo
    let data_len = buf.len();
    let (ai, lamports_box, data_box) = mock_account_info(key, owner, is_signer, is_writable, lamports, data_len);

    data_box.borrow_mut().copy_from_slice(&buf);

    // 将 data_box 放回 ai 的 data 已经在 mock_account_info 内完成，这里只需要覆写内容
    (ai, lamports_box, data_box)
}

/// only for test
pub fn mock_anchor_account_info_v2<'a, 'b, T: ZeroCopy>(
    key: &'a Pubkey,
    owner: &'a Pubkey,
    is_signer: bool,
    is_writable: bool,
    lamports: u64,
    account: &'b T,
    extra_account_data: Option<&[u8]>,
) -> (AccountInfo<'a>, Rc<RefCell<&'a mut u64>>, Rc<RefCell<&'a mut [u8]>>) {
    // 计算 data 长度：8 字节 discriminator + 序列化数据
    let mut buf = Vec::new();
    // 预留 discriminator + 内容
    buf.extend_from_slice(T::DISCRIMINATOR);
    buf.extend_from_slice(bytemuck::bytes_of(account));
    // 追加额外数据
    if let Some(extra_account_data) = extra_account_data {
        buf.extend_from_slice(extra_account_data);
    }

    // 构造 AccountInfo
    let data_len = buf.len();
    let (ai, lamports_box, data_box) = mock_account_info(key, owner, is_signer, is_writable, lamports, data_len);

    data_box.borrow_mut().copy_from_slice(&buf);

    // 将 data_box 放回 ai 的 data 已经在 mock_account_info 内完成，这里只需要覆写内容
    (ai, lamports_box, data_box)
}

pub fn mock_anchor_account_info_v3<'a, 'b, T: ZeroCopy>(
    key: &'a Pubkey,
    owner: &'a Pubkey,
    account: &'b T,
    extra_account_data: Option<&[u8]>,
) -> (AccountInfo<'a>, Rc<RefCell<&'a mut u64>>, Rc<RefCell<&'a mut [u8]>>) {
    mock_anchor_account_info_v2(key, owner, false, true, 0, account, extra_account_data)
}
