use std::process::Command;
use std::env;
use pam_sys::{PamFlag, PamError, PamHandle, PamItemType, get_item};
use std::ffi::CStr;
use std::ptr;

fn create_user(username: &str) -> Result<(), String> {
    let output = Command::new("sudo")
        .arg("useradd")
        .arg(username)
        .output()
        .map_err(|e| format!("Failed to create user: {}", e))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    Ok(())
}

fn delete_user(username: &str) -> Result<(), String> {
    let output = Command::new("sudo")
        .arg("userdel")
        .arg(username)
        .output()
        .map_err(|e| format!("Failed to delete user: {}", e))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    Ok(())
}

#[no_mangle]
extern "C" fn pam_sm_open_session(
    _pamh: *mut PamHandle,
    _flags: PamFlag,
    _argc: i32,
    _argv: *const *const i8,
) -> PamError {
    println!("Session opened.");
    PamError::PAM_SUCCESS
}

#[no_mangle]
extern "C" fn pam_sm_close_session(
    pamh: *mut PamHandle,
    _flags: PamFlag,
    _argc: i32,
    _argv: *const *const i8,
) -> PamError {
    let user_ptr: *const i8 = ptr::null();
    unsafe {
        if get_item(pamh, PamItemType::PAM_USER, &mut user_ptr as *mut *const i8) == PamError::PAM_SUCCESS {
            if !user_ptr.is_null() {
                let username = CStr::from_ptr(user_ptr).to_string_lossy().into_owned();
                if !username.is_empty() {
                    let _ = delete_user(&username);
                    println!("User {} deleted after session.", username);
                }
            }
        }
    }
    PamError::PAM_SUCCESS
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <create|delete> <username>", args[0]);
        return;
    }

    let action = &args[1];
    let username = &args[2];

    match action.as_str() {
        "create" => match create_user(username) {
            Ok(_) => println!("User {} created.", username),
            Err(e) => eprintln!("Error: {}", e),
        },
        "delete" => match delete_user(username) {
            Ok(_) => println!("User {} deleted.", username),
            Err(e) => eprintln!("Error: {}", e),
        },
        _ => eprintln!("Unknown action: {}", action),
    }
}
