| Cell | Rule | Kind | Result | Verdict | Message |
|---|---|---|---|---|---|
| `compiler_alias_after_free` | COMPILER | violation | stops | ok | cannot move out of `data` because it is borrowed |
| `compiler_dangling_return` | COMPILER | violation | stops | ok | cannot return reference to local variable `v` |
| `compiler_double_move` | COMPILER | violation | stops | ok | use of moved value: `a` |
| `compiler_use_after_drop` | COMPILER | violation | stops | ok | borrow of moved value: `data` |
| `context_mut_not_written_private` | CONTEXT | violation | stops | ok | this parameter is a mutable reference but is not used mutably |
| `context_mut_not_written_public` | CONTEXT | gap | passes | ok |  |
| `context_too_many_args` | CONTEXT | violation | stops | ok | this function has too many arguments (8/7) |
| `context_unused_fields` | CONTEXT | gap | passes | ok |  |
| `hidden_impl_drop` | HIDDEN | gap | passes | ok |  |
| `hidden_macro_rules` | HIDDEN | gap | passes | ok |  |
| `hidden_unsafe` | HIDDEN | violation | stops | ok | usage of an `unsafe` block |
| `name_shadow_block` | NAME | violation | stops | ok | `v` shadows a previous, unrelated binding |
| `name_shadow` | NAME | violation | stops | ok | `v` is shadowed |
| `name_sibling_blocks` | NAME | gap | passes | ok |  |
| `name_sibling_for` | NAME | gap | passes | ok |  |
| `release_explicit_drop` | RELEASE | control | passes | ok |  |
| `release_late_drop` | RELEASE | gap | passes | ok |  |
| `release_mem_forget` | RELEASE | violation | stops | ok | usage of `mem::forget` on `Drop` type |
| `state_const_static` | STATE | control | passes | ok |  |
| `state_refcell_param` | STATE | violation | stops | ok | use of a disallowed type `std::cell::RefCell` |
| `state_static_atomic` | STATE | violation | stops | ok | use of a disallowed type `std::sync::atomic::AtomicI32` |
| `state_static_mutex` | STATE | violation | stops | ok | use of a disallowed type `std::sync::Mutex` |
| `state_thread_local` | STATE | violation | stops | ok | use of a disallowed macro `std::thread_local` |
| `world_clock` | WORLD | violation | stops | ok | use of a disallowed method `std::time::Instant::now` |
| `world_env` | WORLD | violation | stops | ok | use of a disallowed method `std::env::var` |
| `world_exit` | WORLD | violation | stops | ok | usage of `process::exit` |
| `world_println` | WORLD | violation | stops | ok | use of a disallowed macro `std::println` |

clippy 0.1.99 (58cf8f9c35 2026-10-13). Wrong verdicts: 0.
