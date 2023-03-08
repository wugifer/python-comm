use {
    std::{
        future::Future,
        pin::Pin,
        task::Poll::{Pending, Ready},
    },
    tokio::macros::support::poll_fn,
};

// std::future::poll_fn 功能相同, 但在某些 rustc 上是非 stable 版本才能用

/// 从 join! 改造而来, 等待 futures 全部完成
/// 1. Pin::new_unchecked 是否正确存疑
pub async fn join_all<F>(mut futures: Vec<F>) -> Vec<F::Output>
where
    F: Future,
{
    // 初始化, poll_fn 内是一个 poll 函数, 会被执行多次, 每次从不同的 future 开始检查
    let mut results: Vec<Option<F::Output>> = futures.iter().map(|_| None).collect();
    let size = futures.len();
    let mut first = 0;

    // 改为引用, 这样可以多次执行 poll_fn + move
    let future_refs = &mut futures;
    let result_refs = &mut results;

    poll_fn(move |cx| {
        // 记录本次 poll_fn 的成果
        let mut is_pending = false;

        for i in 0..size {
            // 每次从不同的 future 开始, 尽管意义似乎不大
            let pos = (first + i) % size;

            // 已经完成的, 不能再次 poll
            if result_refs[pos].is_some() {
                continue;
            }

            // Safety: futures 在栈上, 不会 move. // join! 这么用的, 改成 Vec 后不确定
            let fut = unsafe { Pin::new_unchecked(&mut future_refs[pos]) };

            // 依次 poll, 如果其中一个 ready 并且结果满意, join 完成, 剩余的终止(不再 poll, 但也没有 kill)
            match fut.poll(cx) {
                Pending => {
                    is_pending = true;
                }
                Ready(result) => {
                    result_refs[pos] = Some(result);
                }
            }
        }

        if is_pending {
            // 准备下一次 poll_fn
            first = (first + 1) % size;
            Pending
        } else {
            // 已经全部完成
            Ready(())
        }
    })
    .await;

    // 此时必然全部 Some, 可安全 unwrap
    results.into_iter().map(|x| x.unwrap()).collect()
}

/// 从 join! 改造而来, 等待 futures 全部完成, 按完成顺序 reduce 结果
/// 1. Pin::new_unchecked 是否正确存疑
pub async fn join_all_and_reduce<F, R, A, B>(mut futures: Vec<F>, reduce: R, reduce_args: &A) -> Option<B>
where
    F: Future,
    R: Fn(&mut Option<B>, F::Output, &A, usize) -> (),
{
    // 初始化, poll_fn 内是一个 poll 函数, 会被执行多次, 每次从不同的 future 开始检查
    let mut results: Vec<bool> = futures.iter().map(|_| false).collect();
    let size = futures.len();
    let mut first = 0;
    let mut best = None;

    // 改为引用, 这样可以多次执行 poll_fn + move
    let future_refs = &mut futures;
    let result_refs = &mut results;
    let best_ref = &mut best;

    poll_fn(move |cx| {
        // 记录本次 poll_fn 的成果
        let mut is_pending = false;

        for i in 0..size {
            // 每次从不同的 future 开始, 尽管意义似乎不大
            let pos = (first + i) % size;

            // 已经完成的, 不能再次 poll
            if result_refs[pos] {
                continue;
            }

            // Safety: futures 在栈上, 不会 move. // join! 这么用的, 改成 Vec 后不确定
            let fut = unsafe { Pin::new_unchecked(&mut future_refs[pos]) };

            // 依次 poll, 如果其中一个 ready 并且结果满意, join 完成, 剩余的终止(不再 poll, 但也没有 kill)
            match fut.poll(cx) {
                Pending => {
                    is_pending = true;
                }
                Ready(result) => {
                    result_refs[pos] = true;
                    reduce(best_ref, result, reduce_args, pos);
                }
            }
        }

        if is_pending {
            // 准备下一次 poll_fn
            first = (first + 1) % size;
            Pending
        } else {
            // 已经全部完成
            Ready(())
        }
    })
    .await;

    best
}

/// 从 join! 改造而来, futures 中的部分任务完成后, 如果 is_happy() 返回 true, 结束全部 futures
/// 1. Pin::new_unchecked 是否正确存疑
/// 2. 如果 futures 中有 spawn 返回的``句柄'', 只是结束这个句柄, spawn 内的代码继续执行
/// 3. 返回值 (a, b)
///     1. a: is_happy() 的最终返回值
///     2. b: futures 的全部返回值, 未完成的为 None
pub async fn join_to_happy<F, H, A>(mut futures: Vec<F>, is_happy: H, happy_args: &A) -> (bool, Vec<Option<F::Output>>)
where
    F: Future,
    H: Fn(&Vec<Option<F::Output>>, &A) -> bool,
{
    // 初始化, poll_fn 内是一个 poll 函数, 会被执行多次, 每次从不同的 future 开始检查
    let mut results: Vec<Option<F::Output>> = futures.iter().map(|_| None).collect();
    let size = futures.len();
    let mut first = 0;

    // 改为引用, 这样可以多次执行 poll_fn + move
    let future_refs = &mut futures;
    let result_refs = &mut results;

    let happy = poll_fn(move |cx| {
        // 记录本次 poll_fn 的成果
        let mut is_pending = false;

        for i in 0..size {
            // 每次从不同的 future 开始, 尽管意义似乎不大
            let pos = (first + i) % size;

            // 已经完成的, 不能再次 poll
            if result_refs[pos].is_some() {
                continue;
            }

            // Safety: futures 在栈上, 不会 move. // join! 这么用的, 改成 Vec 后不确定
            let fut = unsafe { Pin::new_unchecked(&mut future_refs[pos]) };

            // 依次 poll, 如果其中一个 ready 并且结果满意, join 完成, 剩余的终止(不再 poll, 但也没有 kill)
            match fut.poll(cx) {
                Pending => {
                    is_pending = true;
                }
                Ready(result) => {
                    result_refs[pos] = Some(result);
                    if is_happy(result_refs, happy_args) {
                        return Ready(true);
                    }
                }
            }
        }

        if is_pending {
            // 准备下一次 poll_fn
            first = (first + 1) % size;
            Pending
        } else {
            // 已经全部完成, 但是 is_happy() 不满足
            Ready(false)
        }
    })
    .await;

    (happy, results)
}
