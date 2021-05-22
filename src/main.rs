#[macro_use] extern crate lazy_static;

pub mod gval;
pub use crate::gval::*;

use std::{borrow::BorrowMut, sync::{Arc}};
use std::cell::{RefMut, RefCell};
use parking_lot::{ReentrantMutex, const_reentrant_mutex};

fn get_first_data_no_arg() -> Arc<ReentrantMutex<RefCell<chord_util::KeyValue>>> {
    let gd_refcell = &*gval::global_datas.lock();
    let gd_refmut = &mut gd_refcell.borrow_mut();
    let kv_arc = gd_refmut.all_data_list.get(0).unwrap().clone();
    return Arc::clone( &kv_arc);
}

fn get_node_from_map(key: &String) -> Arc<ReentrantMutex<RefCell<chord_util::KeyValue>>>{
    let gd_refcell = &*gval::global_datas.lock();
    let gd_refmut = &gd_refcell.borrow_mut();
    let kv_arc = gd_refmut.all_node_dict.get(key).unwrap().clone();
    return Arc::clone(&kv_arc);
}

fn main() {
    {
      let locked_gd : &RefCell<GlobalDatas>  = &*gval::GLOBAL_DATAS.lock();
      {
          let locked_gd_mut : &mut GlobalDatas = &mut locked_gd.borrow_mut();
          //Vecに一つだけデータを追加する
          locked_gd_mut.all_data_list.push(Arc::new(const_reentrant_mutex(RefCell::new(KeyValue::new(Some("ryo_grid".to_string()),"pythonista".to_string())))));

          //ここで locked_gd_mutの参照は無効となるはず
      }

      let first_elem : Arc<ReentrantMutex<RefCell<gval::KeyValue>>>;
      {
          first_elem = get_first_data_no_arg();
          //スコープ外に値を逃がしておく
          first_elem = get_first_data(re_locked_gd_mut);
          
          let first_elem_tmp : &RefCell<KeyValue> = &*first_elem.as_ref().borrow_mut().lock();
          let first_elem_to_print : &mut RefMut<KeyValue> = &mut first_elem_tmp.borrow_mut();

          //この時点でのVec内唯一の要素をprintlnする
          println!("{:?}", first_elem_to_print);

          //ここで参照である first_elem_tmp や first_elem_to_print は無効となるはず
      }

      // first_elem変数に逃がしておいたArc型の値の中身をあれこれしてKeyValue型の
      // mutableな参照を得る
      let locked_elem : &RefCell<KeyValue> = &*first_elem.as_ref().borrow_mut().lock();
      let locked_elem_mut : &mut RefMut<KeyValue> = &mut locked_elem.borrow_mut();
      
      // Vec内に上で追加した要素を可変参照を介して変更する
      locked_elem_mut.value_data = "Rustacean".to_string();

      // 変更された要素をprintlnする
      println!("{:?}", locked_elem_mut);

      // ここで参照である locked_elem、lecked_elem_mut は無効となり、locked_elemに参照を代入
      // する際に獲得した GLOBAL_DATAS のロックも解放されるはず
      // また、同様に、locked_gd に参照を代入する際に獲得した GLOBAL_DATAS のロックも解放されるはず
      // （reentrant なロックを用いているため、同一のデータに対して獲得した2つのロックが解放されるという理解で良い
      //   のだろうか・・・）
    }

    let refcell_gd : &RefCell<GlobalDatas> = &*gval::global_datas.lock();
    {
        let mutref_gd : &mut GlobalDatas = &mut refcell_gd.borrow_mut();
        mutref_gd.all_node_dict.insert(
          "ryo_grid".to_string(), 
          Arc::new(
            const_reentrant_mutex(RefCell::new(KeyValue::new(Some("value".to_string()),"before_mod".to_string())))
          )
        );

        // ここで参照である mutref_gd は無効となるはず
    }    

    let one_elem : Arc<ReentrantMutex<RefCell<chord_util::KeyValue>>>;
    {
        one_elem = get_node_from_map(&"ryo_grid".to_string());
        let one_elem_tmp : &RefCell<KeyValue> = &*one_elem.as_ref().borrow_mut().lock();
        let one_elem_to_print : &mut RefMut<KeyValue> = &mut one_elem_tmp.borrow_mut();

        println!("{:?}", one_elem_to_print);

        // ここで
    }

    let refcell_kv = &*one_elem.as_ref().borrow_mut().lock();
    let mutref_kv = &mut refcell_kv.borrow_mut();
    mutref_kv.value_data = "after_mod".to_string();

    println!("{:?}", mutref_kv);

    // 一応、ここで
}