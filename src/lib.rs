use core::ffi::c_int;
pub mod ffi_mock;

pub use ffi_mock_macro;

#[macro_use]
extern crate lazy_static;

extern "C" {
    fn abs_test(args: c_int) -> c_int;
}

#[cfg(test)]
pub mod tests {

    use std::{slice::Iter, sync::Mutex};

    use super::*;
    use ffi_mock::*;

    #[test]
    pub fn it_works() {
        // let mock = {
        //     lazy_static! {

        //             let a = vec![10];
        //         static ref static_mock: Mutex<FunctionMockInner<c_int, c_int, Iter<'static, c_int>>> =
        //             Mutex::new(FunctionMockInner::new(a.iter()));
        //     }

        //     #[no_mangle]
        //     pub extern "C" fn abs_test(args: c_int) -> c_int {
        //         let mut a = static_mock.lock().unwrap();
        //         a.call_history.push(args);
        //         10
        //     }
        //     FunctionMock::new(&static_mock)
        // };
        let mock: FunctionMock<c_int, c_int> = ffi_mock_macro::mock!(
            fn abs_test(args:c_int) -> c_int
        );
        mock.set_default_return(11);
        mock.add_return(10);

        let b = unsafe { abs_test(10) };
        assert_eq!(b, 10);

        let b = unsafe { abs_test(10) };
        assert_eq!(b, 11);

        let b = unsafe { abs_test(10) };
        assert_eq!(b, 11);

        assert_eq!(mock.calls()[0], 10);
    }
}
