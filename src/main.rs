#[macro_use] extern crate lazy_static;

pub mod gval;
pub use crate::gval::*;

use std::{borrow::BorrowMut, sync::{Arc}};
use std::cell::{RefMut, RefCell};
use parking_lot::{ReentrantMutex, const_reentrant_mutex};

fn get_first_data(gd : RefMut<GlobalDatas>) -> Arc<ReentrantMutex<RefCell<gval::KeyValue>>> {
        let got_data : &Arc<ReentrantMutex<RefCell<KeyValue>>> =  &(*gd.all_data_list.get(0).unwrap());
        let ret: Arc<ReentrantMutex<RefCell<KeyValue>>> = got_data.clone();
        return ret;
}

fn main() {
    let locked_gd = &*gval::GLOBAL_DATAS.lock();
    {
        let locked_gd_mut : &mut GlobalDatas = &mut locked_gd.borrow_mut();
        //Vecに一つだけデータを追加する
        locked_gd_mut.all_data_list.push(Arc::new(const_reentrant_mutex(RefCell::new(KeyValue::new(Some("ryo_grid".to_string()),"pythonista".to_string())))));

        //ここで locked_gd_mutの参照は解放される（はず）
    }

    let re_locked_gd : &RefCell<GlobalDatas> = &*gval::GLOBAL_DATAS.lock();
    let first_elem : Arc<ReentrantMutex<RefCell<gval::KeyValue>>>;
    {
        let re_locked_gd_mut : RefMut<GlobalDatas> = re_locked_gd.borrow_mut();

        //スコープ外に値を逃がしておく
        first_elem = get_first_data(re_locked_gd_mut);
        
        let first_elem_tmp : &RefCell<KeyValue> = &*first_elem.as_ref().borrow_mut().lock();
        let first_elem_to_print : &mut RefMut<KeyValue> = &mut first_elem_tmp.borrow_mut();

        //この時点でのVec内唯一の要素をprintlnする
        println!("{:?}", first_elem_to_print);

        //ここで re_locked_gd_mutの参照は解放される（はず）
    }

    // first_elem変数に逃がしておいたArc型の値の中身をあれこれしてKeyValue型の
    // mutableな参照を得る
    let locked_elem : &RefCell<KeyValue> = &*first_elem.as_ref().borrow_mut().lock();
    let locked_elem_mut : &mut RefMut<KeyValue> = &mut locked_elem.borrow_mut();
    
    // Vec内に上で追加した要素を可変参照を介して変更する
    locked_elem_mut.value_data = "Rustacean".to_string();

    // 変更された要素をprintlnする
    println!("{:?}", locked_elem_mut);
}