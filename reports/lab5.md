# Lab5 实验报告


李骋昊 2021010826

崇祯七甲辰四月廿七日戊戌

### Honor Code

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   -> 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   -> 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 用时

读文档 200min
代码 + 调试  285min
本报告 45min

## 做的更改：

在pcb的inner中维护是否开启检测

新增一个死锁检测模块，对mutex和对信号量的分别作为pcb的inner中的一项，
在`sync.rs`中，在创建mutex和信号量时向死锁检测模块添加资源。在加锁，去锁，P,V时对应维护其中的数据。cond wait视做先去锁再加锁。
死锁检测模块提供一个接口，判断目前是否可能出现死锁，具体算法如文档。

## 问答作业：

#### 在我们的多线程实现中，当主线程 (即 0 号线程) 退出时，视为整个进程退出， 此时需要结束该进程管理的所有线程并回收其资源。 
- 需要回收的资源有哪些？ 
- 其他线程的 TaskControlBlock 可能在哪些位置被引用，分别是否需要回收，为什么？

要回收各个线程通过`TaskUserRes`中持有的资源，从`Drop`方法来看，其中要回收tid, 用户栈占用的物理页，trap context占用的物理页。还要回收通过`TaskControlBlock`持有的内核栈（对应的物理页）。

还需要回收之前的章节中由进程持有的资源,包括持有的fd，以及最后回收整个进程的memory set。

#### 对比以下两种 Mutex.unlock 的实现，二者有什么区别？这些区别可能会导致什么问题？

```rust
impl Mutex for Mutex1 {
    fn unlock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        mutex_inner.locked = false;
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            add_task(waking_task);
        }
    }
}

impl Mutex for Mutex2 {
    fn unlock(&self) {
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            add_task(waking_task);
        } else {
            mutex_inner.locked = false;
        }
    }
}
```

第一种实现中，一个Mutex被解锁时，即使有等待的队列，也会直接释放锁。
第二种实现中，除非此时等待这个Mutex的队列为空，才会释放锁。否则，相当于让队列头部的task继续持有锁。

第一个实现的问题:
这里的等待队列中的任务，都是在调用了`lock`，而锁被占用时被加入到等待的队列里的。也就是说，随后`waiting_task`就会进入临界区。然而，此时对应的mutex锁的locked被设定成false, 有可能有其他的task在这时获取锁，并得以进入临界区，从而导致有两个task同时进入临界区。




