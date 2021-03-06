use fuse::{ReplyAttr, ReplyData, ReplyEntry, Request};

use fs::GoodDataFS;
use fs::inode;

use std::path::Path;

pub fn getattr(_fs: &mut GoodDataFS, _req: &Request, inode: u64, _reply: ReplyAttr) {
    warn!("getattr() - {} {:?}",
          inode,
          inode::Inode::deserialize(inode));
}

pub fn lookup(_fs: &mut GoodDataFS, _req: &Request, parent: u64, name: &Path, _reply: ReplyEntry) {
    warn!("lookup() - {} - {:?}, name: {:?}",
          parent,
          inode::Inode::deserialize(parent),
          name);
}

pub fn read(_fs: &mut GoodDataFS, inode: inode::Inode, _reply: ReplyData, offset: u64, size: u32) {
    warn!("read() - {:?}, offset: {:?}, size; {:?}",
          inode,
          offset,
          size);
}
