use fuse::{FileType, ReplyAttr, ReplyData, ReplyEntry, Request};

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::create_inode_file_attributes;
use fs::item;
use fs::inode;
use helpers;
use object;

use std::path::Path;

use super::project_from_inode;

fn getattr(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let project: &object::Project = &project_from_inode(fs, ino);

    let feature_flags = project.feature_flags(&mut fs.client.connector);
    if feature_flags.is_some() {
        let json: String = feature_flags.unwrap().into();

        let attr =
            create_inode_file_attributes(ino, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.attr(&constants::DEFAULT_TTL, &attr);
    }
}

fn lookup(fs: &mut GoodDataFS, _req: &Request, parent: u64, _name: &Path, reply: ReplyEntry) {
    let inode_parent = inode::Inode::deserialize(parent);
    let inode = inode::Inode::create(inode_parent.project,
                                     constants::Category::Internal as u8,
                                     0,
                                     constants::ReservedFile::FeatureFlagsJson as u8);
    let project: &object::Project = &project_from_inode(fs, inode_parent);

    let feature_flags = project.feature_flags(&mut fs.client.connector);
    if feature_flags.is_some() {
        let json: String = feature_flags.unwrap().into();
        let attr =
            create_inode_file_attributes(inode, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.entry(&constants::DEFAULT_TTL, &attr, 0);
    }
}

fn read(fs: &mut GoodDataFS, inode: inode::Inode, reply: ReplyData, offset: u64, size: u32) {
    debug!("read() - Reading {}",
           constants::FEATURE_FLAGS_JSON_FILENAME);

    let project: &object::Project = &project_from_inode(fs, inode);

    let feature_flags = project.feature_flags(&mut fs.client.connector);
    if feature_flags.is_some() {
        let json: String = feature_flags.unwrap().into();
        reply.data(helpers::read_bytes(&json, offset, size));
    }
}

pub const ITEM: item::ProjectItem = item::ProjectItem {
    category: constants::Category::Internal as u8,
    reserved: constants::ReservedFile::FeatureFlagsJson as u8,
    item_type: FileType::RegularFile,
    path: constants::FEATURE_FLAGS_JSON_FILENAME,

    getattr: getattr,
    lookup: lookup,
    read: read,
};
