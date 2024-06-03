
use core::iter::zip;

use alloc::vec::Vec;
use alloc::vec;

pub struct DeadlockDetect{
    need: Vec<Vec<u32>>,
    allocation : Vec<Vec<u32>>,
    available: Vec<u32>
}

impl DeadlockDetect{
    pub fn new() -> Self{
        Self{
            need : vec![],
            allocation : vec![],
            available : vec![]
        }
    }

    pub fn check_dead_loack(&self) -> bool{
        let mut work = self.available.clone();
        let mut finishes : Vec<bool> = vec![false; work.len()];
        debug!("{:?} {:?} {:?}",&self.need, &self.allocation, &self.available);
        loop{
            let mut flag = false;
            let mut not_finish = false;
            for ((finish, need), allocation) in zip(&mut finishes, &self.need).zip(&self.allocation){
                if *finish {
                    continue;
                }
                not_finish = true;
                assert!(need.len() == work.len());
                assert!(allocation.len() == work.len());
                if zip(need, &work).all(|(n, w)| n <= w){
                    flag = true;
                    for (alloca, work) in zip(allocation, &mut work){
                        *work += *alloca;
                    }
                    *finish = true;
                    break;
                }
            }
            if flag {continue;}
            return not_finish;
        }
    }

    pub fn register_resource(&mut self, rid: usize, init_size: u32){
        
        assert!(self.available.len() <= rid);
        self.available.resize(rid + 1, 0);
        *(self.available.get_mut(rid).unwrap()) = init_size;
        trace!("{:?} {} {}", &self.available, rid, init_size);
        for item in self.need.iter_mut(){
            item.resize(rid + 1, 0);
        }
        for item in self.allocation.iter_mut(){
            item.resize(rid + 1, 0);
        }
    }

    fn allocate_for_tid(&mut self, tid : usize){
        if self.need.len() <= tid {
            self.need.resize(tid + 1, vec![0;self.available.len()]);
        }
        if self.allocation.len() <= tid {
            self.allocation.resize(tid + 1, vec![0;self.available.len()]);
        }
    }

    pub fn modify_available(&mut self, rid: usize, increase: bool){
        trace!("before modify {:?} {}", &self.available, rid);
        if increase{
            *self.available.get_mut(rid).unwrap() += 1;
        }else{
            assert!(*self.available.get(rid).unwrap() != 0);
            *self.available.get_mut(rid).unwrap() -= 1;
        }
        trace!("after modify {:?}", &self.available);
    }

    pub fn modify_need(&mut self, tid: usize, rid: usize, increase: bool){
        self.allocate_for_tid(tid);
        if increase{
            *self.need.get_mut(tid).unwrap().get_mut(rid).unwrap() += 1;
        }else{
            assert!(*self.need.get(tid).unwrap().get(rid).unwrap() != 0);
            *self.need.get_mut(tid).unwrap().get_mut(rid).unwrap() -= 1;
        }
        
    }

    pub fn modify_allocation(&mut self, tid: usize, rid: usize, increase: bool){
        self.allocate_for_tid(tid);
        if increase{
            *self.allocation.get_mut(tid).unwrap().get_mut(rid).unwrap() += 1;
        }else{
            assert!(*self.allocation.get(tid).unwrap().get(rid).unwrap() != 0);
            *self.allocation.get_mut(tid).unwrap().get_mut(rid).unwrap() -= 1;
        }
    }
}