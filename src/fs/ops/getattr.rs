use fuse::{Request, ReplyAttr};
use libc::ENOENT;
use rustc_serialize::json;

use fs::constants;
use fs::GoodDataFS;
use fs::helpers::create_inode_file_attributes;
use fs::flags;
use fs::inode;
use gd;
use object;

pub fn getattr(fs: &mut GoodDataFS, req: &Request, ino: u64, reply: ReplyAttr) {
    let inode = inode::Inode::deserialize(ino);
    println!("GoodDataFS::getattr() - Getting attributes inode {} - {:?}",
             ino,
             inode);

    match ino {
        constants::INODE_ROOT => root(fs, req, ino, reply),
        constants::INODE_PROJECTS => projects(fs, req, ino, reply),
        constants::INODE_PROJECTS_JSON => projects_json(fs, req, ino, reply),
        constants::INODE_USER => user_json(fs, req, ino, reply),
        _ => other(fs, req, ino, reply),
    }
}

fn root(fs: &mut GoodDataFS, _req: &Request, _ino: u64, reply: ReplyAttr) {
    reply.attr(&constants::DEFAULT_TTL, &fs.get_root_dir_attributes())
}

fn projects(fs: &mut GoodDataFS, _req: &Request, _ino: u64, reply: ReplyAttr) {
    reply.attr(&constants::DEFAULT_TTL, &fs.get_projects_dir_attributes())
}

fn projects_json(fs: &mut GoodDataFS, _req: &Request, _ino: u64, reply: ReplyAttr) {
    reply.attr(&constants::DEFAULT_TTL, &fs.get_projects_file_attributes())
}

fn user_json(fs: &mut GoodDataFS, _req: &Request, _ino: u64, reply: ReplyAttr) {
    reply.attr(&constants::DEFAULT_TTL, &fs.get_user_file_attributes())
}

fn project_dir(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    reply.attr(&constants::DEFAULT_TTL, &fs.get_project_dir_attributes(ino))
}

fn project_feature_flags_json(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let inode = inode::Inode::deserialize(ino);
    let pid = (inode.project - 1) as usize;
    let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();

    let feature_flags = project.feature_flags(&mut fs.client);
    if feature_flags.is_some() {
        let json: String = feature_flags.unwrap().into();

        let attr =
            create_inode_file_attributes(ino, json.len() as u64, constants::DEFAULT_CREATE_TIME);
        reply.attr(&constants::DEFAULT_TTL, &attr);
    }
}

fn project_project_json(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let inode = inode::Inode::deserialize(ino);
    let client: &gd::GoodDataClient = fs.client();
    let projects = client.projects().as_ref();
    let json = json::as_pretty_json(&projects.unwrap()[(inode.project - 1) as usize]).to_string();

    let attr = create_inode_file_attributes(ino, json.len() as u64, constants::DEFAULT_CREATE_TIME);
    reply.attr(&constants::DEFAULT_TTL, &attr);
}

fn project_permissions_json(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let inode = inode::Inode::deserialize(ino);
    let pid = (inode.project - 1) as usize;
    let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();
    let json: String = project.user_permissions(&mut fs.client).into();

    let attr = create_inode_file_attributes(ino, json.len() as u64, constants::DEFAULT_CREATE_TIME);
    reply.attr(&constants::DEFAULT_TTL, &attr);
}

fn project_roles_json(fs: &mut GoodDataFS, _req: &Request, ino: u64, reply: ReplyAttr) {
    let inode = inode::Inode::deserialize(ino);
    let pid = (inode.project - 1) as usize;
    let project: &object::Project = &fs.client().projects().as_ref().unwrap()[pid].clone();
    let json: String = project.user_roles(&mut fs.client).into();

    let attr = create_inode_file_attributes(ino, json.len() as u64, constants::DEFAULT_CREATE_TIME);
    reply.attr(&constants::DEFAULT_TTL, &attr);
}

fn other(fs: &mut GoodDataFS, req: &Request, ino: u64, reply: ReplyAttr) {
    let inode = inode::Inode::deserialize(ino);
    if inode.project > 0 {
        if inode.reserved == 0 {
            project_dir(fs, req, ino, reply)
        } else if inode.reserved == flags::ReservedFile::FeatureFlagsJson as u8 {
            project_feature_flags_json(fs, req, ino, reply)
        } else if inode.reserved == flags::ReservedFile::ProjectJson as u8 {
            project_project_json(fs, req, ino, reply)
        } else if inode.reserved == flags::ReservedFile::PermissionsJson as u8 {
            project_permissions_json(fs, req, ino, reply)
        } else if inode.reserved == flags::ReservedFile::RolesJson as u8 {
            project_roles_json(fs, req, ino, reply)
        }
    } else {
        println!("GoodDataFS::getattr() - Not found inode {:?}", ino);
        reply.error(ENOENT);
    }
}