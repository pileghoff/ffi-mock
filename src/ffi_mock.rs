use core::panic;
use std::{
    clone,
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct FunctionMockInner<Tin: Sized + 'static + Clone, Tout: Sized + 'static + Clone> {
    pub call_history: Vec<Tin>,
    pub return_val: VecDeque<Tout>,
    pub default_ret_val: Option<Tout>,
}

impl<Tin, Tout> FunctionMockInner<Tin, Tout>
where
    Tin: Sized + 'static + Clone,
    Tout: Sized + 'static + Clone,
{
    pub fn new() -> Self {
        FunctionMockInner {
            call_history: Vec::new(),
            return_val: VecDeque::new(),
            default_ret_val: None,
        }
    }

    pub fn get_next_return(&mut self) -> Tout {
        match self.return_val.pop_front() {
            Some(v) => v,
            None => match &self.default_ret_val {
                Some(v) => v.clone(),
                None => panic!("Unexpected call"),
            },
        }
    }
}

pub struct FunctionMock<'a, Tin: Sized + 'static + Clone, Tout: Sized + 'static + Clone> {
    pub inner: &'a Mutex<FunctionMockInner<Tin, Tout>>,
}

impl<'a, Tin, Tout> FunctionMock<'a, Tin, Tout>
where
    Tin: Sized + 'static + Clone,
    Tout: Sized + 'static + Clone,
{
    pub fn new(inner: &'a Mutex<FunctionMockInner<Tin, Tout>>) -> Self {
        FunctionMock { inner }
    }

    pub fn calls(self) -> Vec<Tin> {
        self.inner.lock().unwrap().call_history.clone()
    }

    pub fn add_return(&self, val: Tout) {
        self.inner.lock().unwrap().return_val.push_back(val);
    }

    pub fn set_default_return(&self, val: Tout) {
        let mut inner = self.inner.lock().unwrap();
        inner.default_ret_val = Some(val);
    }
}
