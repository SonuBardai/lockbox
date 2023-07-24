- Password manager
  - Use a single master password to protect all the passwords stored in the password manager
  - The master password is used to derive an encryption key, which is then used to encrypt and decrypt the stored passwords. 
- Password store
  - JSON - simple to implement
    - Security - Setup file permissions, Disk encryption
    - Unix systems:
    ```rust
    use std::os::unix::fs::PermissionsExt;
    use std::fs;
    fn set_file_permissions(file_path: &str) -> std::io::Result<()> {
        let mut permissions = fs::metadata(file_path)?.permissions();
        permissions.set_mode(0o600);
        fs::set_permissions(file_path, permissions)
    }
    ```
    - Windows systems: `winapi` (Optional)
    ```rust
    use std::ptr;
    use winapi::um::fileapi::{GetFileAttributesW, SetFileAttributesW};
    use winapi::um::winnt::{FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_NORMAL};
    use std::os::windows::prelude::*;
    use std::ffi::OsStr;
    use std::iter::once;
    use std::io;
    fn set_file_hidden(file_path: &str) -> io::Result<()> {
        let wide_file_path: Vec<u16> = OsStr::new(file_path)
            .encode_wide()
            .chain(once(0))
            .collect();
        let attributes = unsafe { GetFileAttributesW(wide_file_path.as_ptr()) };
        if attributes == u32::MAX {
            return Err(io::Error::last_os_error());
        }
        let new_attributes = if attributes & FILE_ATTRIBUTE_HIDDEN == 0 {
            attributes | FILE_ATTRIBUTE_HIDDEN
        } else {
            attributes & !FILE_ATTRIBUTE_HIDDEN
        };
        let result = unsafe { SetFileAttributesW(wide_file_path.as_ptr(), new_attributes) };
        if result == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    ```
  - Cloud store - Optional
    - Setup support for AWS S3, Azure Blob Storage, Google Cloud Storage, etc. APIs.
