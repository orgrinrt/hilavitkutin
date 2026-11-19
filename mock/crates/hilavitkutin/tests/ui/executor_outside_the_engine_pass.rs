//! A consumer's own executor builds: `Executor` is open, so a stealing
//! executor can take the library's place.

use core::pin::Pin;

use arvo::USize;
use hilavitkutin::thread::{Executor, ExecutorError, PoolFrame, WakeStrategy};

struct StealingExecutor;

impl Executor for StealingExecutor {
    fn run<'frame, 'arena, const C: usize, const P: usize>(
        &self,
        pool: Pin<&'frame PoolFrame<'arena, C, P>>,
        core_id: USize,
        wake_strategy: &WakeStrategy,
    ) -> notko::Outcome<(), ExecutorError> {
        let _ = (pool, core_id, wake_strategy);
        notko::Outcome::Ok(())
    }
}

fn takes_any_executor<E: Executor>(_: &E) {}

fn main() {
    takes_any_executor(&StealingExecutor);
}
