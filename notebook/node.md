# 并发

1. 多线程访问
2. `Arc<T>`, `Mutex<T>`, `RwLock<T>`
3. `DashMap`
4. `AtomicXXX`

在Rust编程语言中，`AtomicXXX`指的是原子操作类型的缩写，这些类型通常用于多线程编程中，以确保在并发环境中对共享数据的安全访问。原子操作可以保证在多个线程之间进行数据操作时，每个操作都是“不可分割”的，即在任何时候，数据要么处于操作前的状态，要么处于操作后的状态，不会出现中间状态。

以下是一些常见的`AtomicXXX`类型：

1. **AtomicBool**：原子布尔类型，用于线程安全的布尔值。

2. **AtomicIsize**、**AtomicI8**、**AtomicI16**、**AtomicI32**、**AtomicI64**、**AtomicI128**：原子有符号整数类型，分别对应不同大小的整数。

3. **AtomicUsize**、**AtomicU8**、**AtomicU16**、**AtomicU32**、**AtomicU64**、**AtomicU128**：原子无符号整数类型，与有符号整数类型类似，但是存储的是无符号整数。

4. **AtomicPtr**：原子指针类型，用于线程安全地共享对任意类型的指针。

这些原子类型都实现了`std::sync::atomic::AtomicXXX`特质（trait），提供了一组原子操作方法，如：

- `load`：安全地读取当前值。
- `store`：安全地设置新值。
- `swap`：原子地交换当前值和新值。
- `compare_and_swap`：比较当前值与给定值，如果相等，则设置新值。
- `fetch_add`、`fetch_sub`、`fetch_and`、`fetch_or`、`fetch_xor`：原子地对当前值进行加、减、位与、位或、位异或操作，并返回操作前的值。

使用原子类型时，Rust提供了内存顺序保证，这些保证定义了在执行原子操作时，内存中其他操作的执行顺序。常见的内存顺序有：

- `SeqCst`：顺序一致性，提供最强的同步效果。
- `Acquire`：获取内存顺序，用于操作前的同步。
- `Release`：释放内存顺序，用于操作后的同步。
- `Relaxed`：放宽的内存顺序，不提供任何同步保证。

原子类型在Rust中是通过`std::sync::atomic`模块提供的，使用时需要引入这个模块。下面是一个简单的使用`AtomicI32`的例子：

```rust
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let atomic = AtomicI32::new(0);
    let arc_atomic = Arc::new(atomic);

    let mut handles = vec![];

    for _ in 0..10 {
        let arc_atomic_clone = Arc::clone(&arc_atomic);
        let handle = thread::spawn(move || {
            let mut sum = 0;
            for i in 0..10 {
                sum += arc_atomic_clone.fetch_add(i as i32, Ordering::SeqCst) + i as i32;
            }
            sum
        });
        handles.push(handle);
    }

    let results: Vec<i32> = handles.into_iter().map(|handle| handle.join().unwrap()).collect();
    let total: i32 = results.iter().sum();

    println!("Total: {}", total);
}
```

在这个例子中，我们创建了一个`AtomicI32`类型的原子变量，并在多个线程中使用`fetch_add`方法进行累加操作。每个线程都会对共享的原子变量执行10次加法操作，并将结果收集到一个向量中。最后，我们计算这些结果的总和，并打印出来。

使用原子类型是多线程编程中处理共享状态的一种安全方式，可以避免竞态条件和数据竞争等问题。

在多线程编程中，内存顺序（memory ordering）是一个关键概念，它定义了在不同线程中执行的操作之间的执行顺序。
Rust中的原子操作支持多种内存顺序，以确保在并发环境中对共享数据的安全访问。以下是对`SeqCst`、`Acquire`、`Release`和`Relaxed`这四种内存顺序的详细介绍：

1. **SeqCst (Sequentially Consistent) 顺序一致性**：
   - `SeqCst`是最强的内存顺序，它保证了所有线程中的操作都会按照某种全局一致的顺序执行，就像它们是在一个单一的全局队列中一样。
   - 使用`SeqCst`内存顺序的原子操作会提供最大的同步效果，确保所有线程都看到一致的状态，但这也意味着可能会有性能上的开销。

2. **Acquire 获取内存顺序**：
   - `Acquire`内存顺序用于确保当前线程中的操作在所有之前的普通内存操作完成后才开始执行，这通常用于读取操作。
   - `Acquire`可以防止编译器或处理器对代码进行重排序，从而确保在当前线程中，所有之前的操作都对其他线程可见。
   - `Acquire`通常用于锁的获取，确保在当前线程中，锁的获取是发生在所有之前的普通内存操作之后。

3. **Release 释放内存顺序**：
   - `Release`内存顺序用于确保当前线程中的操作在所有之后的普通内存操作开始之前完成，这通常用于写入操作。
   - `Release`同样防止了编译器或处理器的重排序，确保在当前线程中，所有之后的操作都对其他线程可见。
   - `Release`通常用于锁的释放，确保在当前线程中，锁的释放是发生在所有之后的普通内存操作之前。

4. **Relaxed 放宽的内存顺序**：
   - `Relaxed`内存顺序不提供任何同步保证，原子操作可以被重排序，也不会影响其他内存操作的执行顺序。
   - 使用`Relaxed`内存顺序的原子操作可以提高性能，因为它减少了编译器和处理器的同步开销。
   - 然而，`Relaxed`内存顺序需要开发者自己确保数据的一致性，只有在操作之间没有依赖关系，或者开发者可以保证数据的一致性时，才应该使用`Relaxed`。

在Rust中，原子操作的内存顺序是通过`Ordering`枚举来指定的。开发者可以根据具体的同步需求选择合适的内存顺序。

例如，如果需要确保两个线程之间的操作顺序，可以使用`Acquire`和`Release`来实现；
如果需要全局的一致性保证，则使用`SeqCst`。而当操作之间没有数据依赖，且不需要保证其他线程的可见性时，可以使用`Relaxed`来提高性能。

[无畏并发]
https://doc.rust-lang.org/book/ch16-01-threads.html#using-threads-to-run-code-simultaneously
