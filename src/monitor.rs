extern crate winapi;

use winapi::um::winuser::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;

use std::io::Error;
use std::mem;
use std::ptr;

pub struct Monitor {
	pub name: String,
	pub w: i32,
	pub h: i32,
	pub x: i32,
	pub y: i32
}

pub fn monitors_list(
) -> Vec<Monitor> {
	let mut monitors = Vec::<Monitor>::new();
	let userdata = &mut monitors as *mut _;

	let result = unsafe {
		EnumDisplayMonitors(
			ptr::null_mut(),
			ptr::null(),
			Some(enumerate_monitors_callback),
			userdata as LPARAM
		)
	};

	if result != TRUE {
		// Get the last error for the current thread.
		// This is analogous to calling the Win32 API GetLastError.
		panic!("Could not enumerate monitors: {}", Error::last_os_error());
	}

	monitors
}

unsafe extern "system" fn enumerate_monitors_callback(
	monitor: HMONITOR,
	_: HDC,
	_: LPRECT,
	userdata: LPARAM,
) -> BOOL {
	let monitors: &mut Vec<Monitor> = mem::transmute(userdata);

	let mut monitor_info: MONITORINFOEXW = mem::zeroed();
	monitor_info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
	let monitor_info_ptr = <*mut _>::cast(&mut monitor_info);

	let result = GetMonitorInfoW(monitor, monitor_info_ptr);
	if result == TRUE {
		let work_area: &RECT = &monitor_info.rcWork;
		println!("left: {}, right: {}, top: {}, bottom: {}", work_area.left, work_area.right, work_area.top, work_area.bottom);
		monitors.push(Monitor {
			name: String::from_utf16(&monitor_info.szDevice).unwrap(),
			w: (work_area.left - work_area.right).abs(),
			h: (work_area.top - work_area.bottom).abs(),
			x: work_area.left,
			y: work_area.top
		});
	}

	TRUE
}
