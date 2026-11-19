//! An executor written outside the engine is refused: `Executor` takes
//! the engine's seal, which no outside crate can name or implement.

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

fn main() {}
