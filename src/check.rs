use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::PermissionsExt;

pub fn metadata(obj: &str, meta: &std::fs::Metadata, requested_type: &str, mode: u32, ok: &mut Vec<String>, warn: &mut Vec<String>, crit: &mut Vec<String>, report_as_crit: bool) {
    let ftext = meta.file_type();
    let perm = meta.permissions().mode() & 0x0fff; // drop file type bit field

    match requested_type {
        "F" => {
            if ftext.is_fifo() {
                if perm == mode {
                    ok.push(format!("{} is a fifo with correct permissions", obj));
                } else {
                    if report_as_crit {
                        crit.push(format!("{} is a fifo but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    } else {
                        warn.push(format!("{} is a fifo but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    }
                }
            } else {
                if report_as_crit {
                    crit.push(format!("{} is not a fifo but a {}", obj, get_type_name(meta)));
                } else {
                    warn.push(format!("{} is not a fifo but a {}", obj, get_type_name(meta)));
                }
            }
        },
        "b" => {
            if ftext.is_block_device() {
                if perm == mode {
                    ok.push(format!("{} is a block device with correct permissions", obj));
                } else {
                    if report_as_crit {
                        crit.push(format!("{} is a block device but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    } else {
                        warn.push(format!("{} is a block device but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    }
                }
            } else {
                if report_as_crit {
                    crit.push(format!("{} is not a block device but a {}", obj, get_type_name(meta)));
                } else {
                    warn.push(format!("{} is not a block device but a {}", obj, get_type_name(meta)));
                }
            }
        }
        "c" => {
            if ftext.is_char_device() {
                if perm == mode {
                    ok.push(format!("{} is a char device with correct permissions", obj));
                } else {
                    if report_as_crit {
                        crit.push(format!("{} is a char device but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    } else {
                        warn.push(format!("{} is a char device but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    }
                }
            } else {
                if report_as_crit {
                    crit.push(format!("{} is not a char device but a {}", obj, get_type_name(meta)));
                } else {
                    warn.push(format!("{} is not a char device but a {}", obj, get_type_name(meta)));
                }
            }
        },
        "d" => {
            if ftext.is_dir() {
                if perm == mode {
                    ok.push(format!("{} is a directory with correct permissions", obj));
                } else {
                    if report_as_crit {
                        crit.push(format!("{} is a directory but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    } else {
                        warn.push(format!("{} is a directory but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    }
                }
            } else {
                if report_as_crit {
                    crit.push(format!("{} is not a directory but a {}", obj, get_type_name(meta)));
                } else {
                    warn.push(format!("{} is not a directory but a {}", obj, get_type_name(meta)));
                }
            }
        },
        "f" => {
            if ftext.is_file() {
                if perm == mode {
                    ok.push(format!("{} is a file with correct permissions", obj));
                } else {
                    if report_as_crit {
                        crit.push(format!("{} is a file but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    } else {
                        warn.push(format!("{} is a file but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    }
                }
            } else {
                if report_as_crit {
                    crit.push(format!("{} is not a file but a {}", obj, get_type_name(meta)));
                } else {
                    warn.push(format!("{} is not a file but a {}", obj, get_type_name(meta)));
                }
            }
        },
        "l" => {
            if ftext.is_symlink() {
                if perm == mode {
                    ok.push(format!("{} is a symbolic link with correct permissions", obj));
                } else {
                    if report_as_crit {
                        crit.push(format!("{} is a symbolic link but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    } else {
                        warn.push(format!("{} is a symbolic link but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    }
                }
            } else {
                if report_as_crit {
                    crit.push(format!("{} is not a symbolic link but a {}", obj, get_type_name(meta)));
                } else {
                    warn.push(format!("{} is not a symbolic link but a {}", obj, get_type_name(meta)));
                }
            }
        },
        "s" => {
            if ftext.is_socket() {
                if perm == mode {
                    ok.push(format!("{} is a socket with correct permissions", obj));
                } else {
                    if report_as_crit {
                        crit.push(format!("{} is a socket but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    } else {
                        warn.push(format!("{} is a socket but permissions are incorrect - {:#06o} instead of {:#06o}", obj, perm, mode));
                    }
                }
            } else {
                if report_as_crit {
                    crit.push(format!("{} is not a socket but a {}", obj, get_type_name(meta)));
                } else {
                    warn.push(format!("{} is not a socket but a {}", obj, get_type_name(meta)));
                }
            }
        },
        _ => {
            panic!("BUG: {}:{}: Invalid type {}", file!(), line!(), requested_type);
        }
    };
}


fn get_type_name(meta: &std::fs::Metadata) -> &str {
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

