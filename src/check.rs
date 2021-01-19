use crate::fsobj;

use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;

pub fn metadata(
    obj: fsobj::FSObj,
    ok: &mut Vec<String>,
    warn: &mut Vec<String>,
    crit: &mut Vec<String>,
    report_as_crit: bool,
) {
    // pub fn metadata(obj: &str, meta: &std::fs::Metadata, requested_type: &str, mode: u32, ok: &mut Vec<String>, warn: &mut Vec<String>, crit: &mut Vec<String>, report_as_crit: bool) {
    let ftext = obj.meta.file_type();
    let perm = obj.meta.permissions().mode() & 0x0fff; // drop file type bit field

    match obj.requested_type.as_str() {
        "F" => {
            if ftext.is_fifo() {
                if perm == obj.mode {
                    ok.push(format!("{} is a fifo with correct permissions", obj.name));
                } else if report_as_crit {
                    crit.push(format!(
                        "{} is a fifo but permissions are incorrect - {:#06o} instead of {:#06o}",
                        obj.name, perm, obj.mode
                    ));
                } else {
                    warn.push(format!(
                        "{} is a fifo but permissions are incorrect - {:#06o} instead of {:#06o}",
                        obj.name, perm, obj.mode
                    ));
                }
            } else if report_as_crit {
                crit.push(format!(
                    "{} is not a fifo but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            } else {
                warn.push(format!(
                    "{} is not a fifo but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            }
        }
        "b" => {
            if ftext.is_block_device() {
                if perm == obj.mode {
                    ok.push(format!(
                        "{} is a block device with correct permissions",
                        obj.name
                    ));
                } else if report_as_crit {
                    crit.push(format!("{} is a block device but permissions are incorrect - {:#06o} instead of {:#06o}", obj.name, perm, obj.mode));
                } else {
                    warn.push(format!("{} is a block device but permissions are incorrect - {:#06o} instead of {:#06o}", obj.name, perm, obj.mode));
                }
            } else if report_as_crit {
                crit.push(format!(
                    "{} is not a block device but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            } else {
                warn.push(format!(
                    "{} is not a block device but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            }
        }
        "c" => {
            if ftext.is_char_device() {
                if perm == obj.mode {
                    ok.push(format!(
                        "{} is a char device with correct permissions",
                        obj.name
                    ));
                } else if report_as_crit {
                    crit.push(format!("{} is a char device but permissions are incorrect - {:#06o} instead of {:#06o}", obj.name, perm, obj.mode));
                } else {
                    warn.push(format!("{} is a char device but permissions are incorrect - {:#06o} instead of {:#06o}", obj.name, perm, obj.mode));
                }
            } else if report_as_crit {
                crit.push(format!(
                    "{} is not a char device but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            } else {
                warn.push(format!(
                    "{} is not a char device but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            }
        }
        "d" => {
            if ftext.is_dir() {
                if perm == obj.mode {
                    ok.push(format!(
                        "{} is a directory with correct permissions",
                        obj.name
                    ));
                } else if report_as_crit {
                    crit.push(format!("{} is a directory but permissions are incorrect - {:#06o} instead of {:#06o}", obj.name, perm, obj.mode));
                } else {
                    warn.push(format!("{} is a directory but permissions are incorrect - {:#06o} instead of {:#06o}", obj.name, perm, obj.mode));
                }
            } else if report_as_crit {
                crit.push(format!(
                    "{} is not a directory but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            } else {
                warn.push(format!(
                    "{} is not a directory but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            }
        }
        "f" => {
            if ftext.is_file() {
                if perm == obj.mode {
                    ok.push(format!("{} is a file with correct permissions", obj.name));
                } else if report_as_crit {
                    crit.push(format!(
                        "{} is a file but permissions are incorrect - {:#06o} instead of {:#06o}",
                        obj.name, perm, obj.mode
                    ));
                } else {
                    warn.push(format!(
                        "{} is a file but permissions are incorrect - {:#06o} instead of {:#06o}",
                        obj.name, perm, obj.mode
                    ));
                }
            } else if report_as_crit {
                crit.push(format!(
                    "{} is not a file but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            } else {
                warn.push(format!(
                    "{} is not a file but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            }
        }
        "l" => {
            if ftext.is_symlink() {
                if perm == obj.mode {
                    ok.push(format!(
                        "{} is a symbolic link with correct permissions",
                        obj.name
                    ));
                } else if report_as_crit {
                    crit.push(format!("{} is a symbolic link but permissions are incorrect - {:#06o} instead of {:#06o}", obj.name, perm, obj.mode));
                } else {
                    warn.push(format!("{} is a symbolic link but permissions are incorrect - {:#06o} instead of {:#06o}", obj.name, perm, obj.mode));
                }
            } else if report_as_crit {
                crit.push(format!(
                    "{} is not a symbolic link but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            } else {
                warn.push(format!(
                    "{} is not a symbolic link but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            }
        }
        "s" => {
            if ftext.is_socket() {
                if perm == obj.mode {
                    ok.push(format!("{} is a socket with correct permissions", obj.name));
                } else if report_as_crit {
                    crit.push(format!(
                        "{} is a socket but permissions are incorrect - {:#06o} instead of {:#06o}",
                        obj.name, perm, obj.mode
                    ));
                } else {
                    warn.push(format!(
                        "{} is a socket but permissions are incorrect - {:#06o} instead of {:#06o}",
                        obj.name, perm, obj.mode
                    ));
                }
            } else if report_as_crit {
                crit.push(format!(
                    "{} is not a socket but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            } else {
                warn.push(format!(
                    "{} is not a socket but a {}",
                    obj.name,
                    get_type_name(obj.meta)
                ));
            }
        }
        _ => {
            panic!(
                "BUG: {}:{}: Invalid type {}",
                file!(),
                line!(),
                obj.requested_type
            );
        }
    };
}

fn get_type_name(meta: std::fs::Metadata) -> &'static str {
    let ft = meta.file_type();

    if ft.is_fifo() {
        "fifo"
    } else if ft.is_block_device() {
        "block device"
    } else if ft.is_char_device() {
        "char device"
    } else if ft.is_dir() {
        "directory"
    } else if ft.is_file() {
        "file"
    } else if ft.is_symlink() {
        "symbolic link"
    } else if ft.is_socket() {
        "socket"
    } else {
        "???"
    }
}
