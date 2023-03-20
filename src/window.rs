extern crate winapi;
extern crate alloc;

use winapi::um::winuser::*;
use winapi::um::wingdi::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;

use alloc::ffi::CString;
use std::collections::HashMap;
use std::ptr;

pub struct WindowInfo {
	hwnd: HWND,
	hdc: HDC,
	hglrc: HGLRC,
	pub width: i32,
	pub height: i32,
	pub x: i32,
	pub y: i32
}

pub fn window_background_create() -> Result<WindowInfo, String> {

	// Find the Compositor Window by its class name
	let class_name: CString = CString::new("Progman").unwrap();
	let window_name: CString = CString::new("Program Manager").unwrap();
	let progman: HWND = unsafe { FindWindowA(class_name.as_ptr(), window_name.as_ptr()) };
	if progman == ptr::null_mut() {
		panic!("Failed to find DWM compositor window");
	}

	unsafe {
		SendMessageTimeoutA(
			progman,
			0x052C,
			0,
			0,
			SMTO_NORMAL,
			1000,
			ptr::null_mut());
	}

	let mut hwnd: HWND = ptr::null_mut();
	unsafe { EnumWindows(Some(enum_windows_proc), (&mut hwnd) as *mut HWND as isize); }

	if hwnd == ptr::null_mut() {
		panic!("Failed to get WorkerW window.");
	}

	let mut rect: RECT = RECT { bottom: 0, left: 0, right: 0, top: 0 };
	if unsafe { GetWindowRect(hwnd, &mut rect as *mut RECT) == 0 } {
		return Err("Failed to get window RECT.".to_string());
	}
	let width = rect.right - rect.left;
	let height = rect.bottom - rect.top;

	let hdc: HDC = unsafe { GetDC(hwnd) };

	if hdc.is_null() {
		return Err("Failed to get device context".to_string());
	}

	let mut pfd: PIXELFORMATDESCRIPTOR = unsafe { std::mem::zeroed() };
	pfd.nSize = std::mem::size_of::<winapi::um::wingdi::PIXELFORMATDESCRIPTOR>() as u16;
	pfd.nVersion = 1;
	pfd.dwFlags = winapi::um::wingdi::PFD_DRAW_TO_WINDOW | winapi::um::wingdi::PFD_SUPPORT_OPENGL | winapi::um::wingdi::PFD_DOUBLEBUFFER;
	pfd.iPixelType = winapi::um::wingdi::PFD_TYPE_RGBA;
	pfd.cColorBits = 32;
	pfd.cDepthBits = 24;
	pfd.cStencilBits = 8;

	let pixel_format = unsafe { ChoosePixelFormat(hdc, &pfd) };
	if pixel_format == 0 {
		return Err("Failed to choose pixel format".to_string());
	}

	if unsafe { SetPixelFormat(hdc, pixel_format, &pfd) } == 0 {
		return Err("Failed to set pixel format".to_string());
	}

	let hglrc: HGLRC = unsafe { winapi::um::wingdi::wglCreateContext(hdc) };
	if hglrc.is_null() {
		return Err("Failed to create OpenGL context".to_string());
	}

	if unsafe { winapi::um::wingdi::wglMakeCurrent(hdc, hglrc) } == 0 {
		return Err("Failed to make OpenGL context current".to_string());
	}

	gl_loader::init_gl();
	gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);

	unsafe { gl::Enable(gl::DEBUG_OUTPUT) };
	unsafe { gl::DebugMessageCallback(Some(gl_error_callback), ptr::null()) };

	println!("GL_VERSION: {}", unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8).to_str().unwrap() });

	Ok(WindowInfo {
		hwnd: hwnd,
		hdc: hdc,
		hglrc: hglrc,
		width: width,
		height: height,
		x: rect.left,
		y: rect.top
	})
}

pub fn window_background_destroy(window: WindowInfo) -> () {
	gl_loader::end_gl();
	unsafe { winapi::um::wingdi::wglDeleteContext(window.hglrc) };
	unsafe { ReleaseDC(window.hwnd, window.hdc) };
	//unsafe { CloseWindow(window.hwnd) };
}

pub fn window_swap_buffers(window: &WindowInfo) -> () {
	unsafe { SwapBuffers(window.hdc) };
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
	let class_name: CString = CString::new("SHELLDLL_DefView").unwrap();
	let p: HWND = FindWindowExA(
		hwnd,
		ptr::null_mut(),
		class_name.as_ptr() as *const i8,
		ptr::null_mut());

	if p != ptr::null_mut() {
		let class_name = CString::new("WorkerW").unwrap();
		let hwnd_out_ptr: *mut HWND = lparam as *mut HWND;
		*hwnd_out_ptr = FindWindowExA(
			ptr::null_mut(),
			hwnd,
			class_name.as_ptr() as *const i8,
			ptr::null_mut());
		0
	} else {
		1
	}
}

use gl::types::*;

extern "system" fn gl_error_callback(_source: GLenum, _gltype: GLenum, _id: GLuint, severity: GLenum, _length: GLsizei, message: *const GLchar, _user_param: *mut std::ffi::c_void) {
	let severity_names: HashMap<GLenum, &str> = HashMap::<GLenum, &str>::from_iter([
		(gl::DEBUG_SEVERITY_HIGH, "ERROR"),
		(gl::DEBUG_SEVERITY_MEDIUM, "WARNING"),
		(gl::DEBUG_SEVERITY_LOW, "INFO"),
		(gl::DEBUG_SEVERITY_NOTIFICATION, "TRACE")
		]);
	println!("[{}] {}", severity_names[&severity], unsafe { std::ffi::CStr::from_ptr(message).to_str().unwrap() });
}
