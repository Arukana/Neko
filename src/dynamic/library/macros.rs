#[macro_export]
macro_rules! symbol {
   ($handle: expr, $symbol: expr) => ({
       let sym: *mut libc::c_void = libc::dlsym($handle, $symbol);

       if sym.ne(&ptr::null_mut()) {
           Some(mem::transmute::<*mut libc::c_void, _>(sym))
       } else {
           None
       }
   });
}
