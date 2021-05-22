use std::sync::{Arc};
use std::cell::RefCell;
use parking_lot::{ReentrantMutex, const_reentrant_mutex};

pub struct GlobalDatas {
    pub all_data_list : Vec<Arc<ReentrantMutex<RefCell<KeyValue>>>>
    //他のメンバも本来は存在するがこのスニペットでは省略する
}

impl GlobalDatas {
    pub fn new() -> GlobalDatas {
        GlobalDatas {all_data_list : Vec::new()}
    }
}

lazy_static! {
    pub static ref GLOBAL_DATAS : Arc<ReentrantMutex<RefCell<GlobalDatas>>> = Arc::new(const_reentrant_mutex(RefCell::new(GlobalDatas::new())));
}

#[derive(Debug, Clone)]
pub struct KeyValue {
    pub key : Option<String>,
    pub value_data : String,
    pub data_id : Option<i32>
}

impl KeyValue {
    pub fn new(key : Option<String>, value : String) -> KeyValue {
        let tmp_data_id : Option<i32> = match &key {
            Some(key_string) => Some(hash_str_to_int(&key_string)),
            None => None
        };
        KeyValue {key : key, value_data : value, data_id : tmp_data_id}
    }
}

pub fn hash_str_to_int(_input_str : &String) -> i32 {
    //return fixed value
    return 1000
}