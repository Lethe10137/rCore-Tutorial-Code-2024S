# Lab1 实验报告

李骋昊 2021010826

崇祯七甲辰二月十二日甲申

### Honor Code

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > 陈英豪同学。 讨论了整体思路，由于不涉及具体的代码，没有办法具体注释。

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   -> 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。





https://learningos.cn/rCore-Tutorial-Guide-2024S/chapter3/5exercise.html

## 做的更改：

1. 把`TaskInfo`中的若干字段改成`pub`
2. 在`TaskControlBlock`中新增`starttime`和`sys_call_times`字段，并添加初始化
3. 在启动task时维护`starttime`
4. 在执行系统调用前维护`sys_call_times`
5. 实现获取`TaskInfo`的调用。其中time利用`starttime`和`get_time_ms()`计算，`syscall_times`直接返回

## 问答作业：

### 第一题

使用的`sbi`和版本：

`[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0`



```
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

`bad_address`中尝试对地址`0x0`处写入，所以表现为`PageFault`

而`bad_instruction`和`bad_register` 都表现为一个`IllegalInstruction`。这是因为两者分别使用了`sret`和`csrr`这两个在U态不能使用的指令。

### 第2题

####  进入`__restore`时  `a0`的值。指出`__restore`的两种使用场景



值： 这时的值就是`trap_handler`的返回值。即kernel stack 上的`TrapContext`的指针。

场景：

1. 第一次进入用户态时
2. 从一个`trap`中返回对用户态时

####  `L43-L48`

```asm
ld t0, 32*8(sp)
ld t1, 33*8(sp)
ld t2, 2*8(sp)
csrw sstatus, t0
csrw sepc, t1
csrw sscratch, t2
```

这里处理的是`sstatus`, `sepc`, `sscratch` 三个csr寄存器。

`sstatus`被设定成内核栈上的`TrapContext`中的`sstatus`字段。这样恢复了`SPP`等字段，从而在随后的`sret`中恢复了特权级为U。

`sepc`被设定成内核栈上的`TrapContext`中的`sstatus`字段,其中记录了要返回后执行的地址。从而在随后的`sret`中回到进trap之前的地址。

`sscratch`此时被恢复成成用户栈的栈指针，以在`csrrw sp, sscratch, sp`过程中完成换栈。



####  `L50-L56`

`x2`是`sp`, 要等到`csrrw sp, sscratch, sp`再恢复。`x4`是`tp`，这个寄存器应用程序不使用。所以不用恢复。



#### `L60`

之后， `sp`是用户栈的栈顶，而`sscratch`中是内核栈的栈顶。



#### `__restore`中的状态切换出现在哪一条指令？

`sret`

此前`sstatus`已经恢复成了`TrapContext`中的值，即其中的`SPP`字段是`U`。所以在执行`sret`后会来到`U`态。



#### `_L13`

之后， `sp`是内核栈的栈顶，而`sscratch`中是用户栈的栈顶。



#### 哪条指令中进入S态？

在用户态程序中的`ecall`中进入`S`态。或者发生了被委托到`S`态的中断或异常时进入。它的地址被记录在`sepc`中（在随后进入`trap_handler`后可能被修改成下一条指令。）
