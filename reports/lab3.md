# Lab3 实验报告


李骋昊 2021010826

崇祯七甲辰三月十六日戊午

### Honor Code

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   -> 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   -> 无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。



## 做的更改：

仿照fork和execute实现spawn系统调用
实现stride算法，在task manager的fetch方法中维护stride

## 问答作业：

实际情况不是轮到p1执行。
在p2执行一个时间片之后，p2的stride值变成了(250 + 25) mod 256 = 19
于是 p1的stride比p2的stride要大，所以还是p2执行


考虑归纳法。初始的时候各个进程的stride值是0，满足条件。
考虑在第$t$个时间片后，stride最大的进程的stride值， 记为$S_{max}^{(t)}$
stride最小的进程的stride的stride值是$S_{min}$， 有 $S_{max}^{(t)} \leq S_{min}^{(t)} + \dfrac{BigStride}{2}$
下一个时间片后，stride最大的进程要么和$t$相同，要么是刚刚调度了的进程，故$S_{max}^{(t+1)} = max(S_{min}^{(t)} + \dfrac{BigStride}{pass}, S_{max}^{(t)}) \leq max(S_{min}^{(t)} + \dfrac{BigStride}{2}, S_{max}^{(t)}) =S_{min}^{(t)} + \dfrac{BigStride}{2}$

显然有$S_{min}^{(t+1)} + \dfrac{BigStride}{2} \geq S_{min}^{(t)} + \dfrac{BigStride}{2} \geq S_{max}^{(t+1)}$，于是对$t+1$的情形也成立

```rust
impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 > other.0{
            if self.0 - other.0 > 128{
                Some(Ordering::Less)
            }else{
                Some(Ordering::Greater)
            }
        } else {
            if other.0 - self.0 <= 128{
                Some(Ordering::Less)
            }else{
                Some(Ordering::Greater)
            }
        }
    }
}


```



