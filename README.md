**_Note:_** Because I'm running my own servers for several years, main development is done at at https://git.ypbind.de/cgit/check_fs_object/

----

# Preface
`check_fs_object` is a Nagios check to check for the type and mode of a filesystem object

# Build requirements
A recent version of [Rust](https://www.rust-lang.org) and [Cargo](https://crates.io/)

# Command line parameters
In addition to the command line parameters a list of filesystem objects to check must be provided.

| *Parameter* | *Description* | *Comment* |
|:------------|:--------------|:----------|
| `-C` / `--critical` | Report CRITICAL state instead of WARNING state if either type or mode doesn't match | - |
| `-V` / `--version` | Show version information | - |
| `-h` / `--help` | Show help text | - |
| `-m <mode>` / `--mode=<mode>` | Required mode of file system object as octal value | This parameter is mandatory |
| `-t <type>` / `--type=<type>` | Required type of file system object | This parameter is mandatory |
|                               |  F - fifo | |
|                               |  b - block device |  |
|                               |  c - char device |  |
|                               |  d - directory |  |
|                               |  f - file |  |
|                               |  l - symbolic link |  |
|                               |  s - socket |  |
| `-f` / `--follow`             | Follow symbolic link and check type and mode of it's target | |


For example, check if `/tmp` and `/var/tmp` are directories with the correct permissions (`rwxrwxrwxt` - octal `1777`) run `check_fs_object --mode=1777 --type=d /tmp /var/tmp`

To check for a broken symbolic link, use the `-f` / `--follow` option. If the destination does not exist `No such file or directory` will be reported.

# License
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.


