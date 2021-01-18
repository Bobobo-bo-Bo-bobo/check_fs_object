use crate::constants;

pub fn show_version() {
    println!("{} version {}
Copyright (C) 2021 by Andreas Maus <maus@ypbind.de>
This program comes with ABSOLUTELY NO WARRANTY.

{} is distributed under the Terms of the GNU General
Public License Version 3. (http://www.gnu.org/copyleft/gpl.html)
",
        constants::NAME,
        constants::VERSION,
        constants::NAME
    );

}

pub fn show_usage() {
    show_version();
    println!("Usage: {} [-C|--critical] [-V|--version] [-h|--help] [-m <octal>|--mode=<octal>] [-t <type>|--type=<type>] <fs_obj> <fs_obj> ...

    -C              If mode/type doesn't match report CRITICAL status
    --critical      instead of WARNING status

    -V              Show version information
    --version

    -h              Show help text
    --help

    -m <octal>      Required mode of file system object as octal value
    --mode=<octal>

    -t <type>       Required type of file system object
    --type=<type>   Type can be one of:
                        F - fifo
                        b - block device
                        c - char device
                        d - directory
                        f - file
                        l - symbolic link
                        s - socket

    -f              Follow symbolic link
    --follow

Example:
    Check if /tmp and /var/tmp are a directory and has mode 1777
        {} --type=d --mode=1777 /tmp /var/tmp

", constants::NAME, constants::NAME);
}

