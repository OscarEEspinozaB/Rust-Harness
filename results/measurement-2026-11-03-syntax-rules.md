| Cell | Rule | Kind | Result | Verdict | Message |
|---|---|---|---|---|---|
| `bypass_allow_crate` | BYPASS | violation | stops | ok | [BYPASS] `clippy::disallowed_methods` is a rule of the harness and can |
| `bypass_cfg_attr` | BYPASS | violation | stops | ok | [BYPASS] `unsafe_code` is a rule of the harness and cannot be relaxed  |
| `bypass_group` | BYPASS | violation | stops | ok | [BYPASS] `warnings` is a rule of the harness and cannot be relaxed her |
| `bypass_main_relaxes_other_rule` | BYPASS | violation | stops | ok | [BYPASS] `unsafe_code` is a rule of the harness and cannot be relaxed  |
| `bypass_other_lint_is_free` | BYPASS | control | passes | ok |  |
| `compiler_alias_after_free` | COMPILER | violation | stops | ok | cannot move out of `data` because it is borrowed |
| `compiler_dangling_return` | COMPILER | violation | stops | ok | cannot return reference to local variable `v` |
| `compiler_double_move` | COMPILER | violation | stops | ok | use of moved value: `a` |
| `compiler_use_after_drop` | COMPILER | violation | stops | ok | borrow of moved value: `data` |
| `context_mut_not_written_private` | CONTEXT | violation | stops | ok | this parameter is a mutable reference but is not used mutably |
| `context_mut_not_written_public` | CONTEXT | violation | stops | ok | this parameter is a mutable reference but is not used mutably |
| `context_mut_reborrow_unmarked` | CONTEXT | gap | passes | ok |  |
| `context_too_many_args` | CONTEXT | violation | stops | ok | [NAME] parameter `f` has the name of its function |
| `context_unused_fields` | CONTEXT | gap | passes | ok |  |
| `hidden_impl_drop` | HIDDEN | gap | passes | ok |  |
| `hidden_macro_rules` | HIDDEN | gap | passes | ok |  |
| `hidden_unsafe` | HIDDEN | violation | stops | ok | usage of an `unsafe` block |
| `lambda_assigns_field` | LAMBDA | violation | stops | ok | [LAMBDA] closure writes `state`, a name it did not declare |
| `lambda_borrows_capture_mutably` | LAMBDA | violation | stops | ok | [LAMBDA] closure writes `list`, a name it did not declare |
| `lambda_inner_writes_outer_local` | LAMBDA | violation | stops | ok | [LAMBDA] closure writes `count`, a name it did not declare |
| `lambda_mutates_through_method` | LAMBDA | gap | passes | ok |  |
| `lambda_reads_capture` | LAMBDA | control | passes | ok |  |
| `lambda_writes_capture` | LAMBDA | violation | stops | ok | [LAMBDA] closure writes `total`, a name it did not declare |
| `lambda_writes_own` | LAMBDA | control | passes | ok |  |
| `name_local_as_method` | NAME | control | passes | ok |  |
| `name_parameter_as_function` | NAME | violation | stops | ok | [NAME] parameter `total` has the name of its function |
| `name_shadow_block` | NAME | violation | stops | ok | `v` shadows a previous, unrelated binding |
| `name_shadow` | NAME | violation | stops | ok | `v` is shadowed |
| `name_sibling_blocks` | NAME | control | passes | ok |  |
| `name_sibling_for` | NAME | control | passes | ok |  |
| `name_variable_and_function` | NAME | violation | stops | ok | [NAME] `step` names a variable and a function inside `walk` |
| `release_explicit_drop` | RELEASE | control | passes | ok |  |
| `release_late_drop` | RELEASE | gap | passes | ok |  |
| `release_mem_forget` | RELEASE | violation | stops | ok | usage of `mem::forget` on `Drop` type |
| `state_atomic_ptr` | STATE | violation | stops | ok | use of a disallowed type `std::sync::atomic::AtomicPtr` |
| `state_atomic_u8` | STATE | violation | stops | ok | use of a disallowed type `std::sync::atomic::AtomicU8` |
| `state_const_static` | STATE | control | passes | ok |  |
| `state_global_allocator` | STATE | control | passes | ok |  |
| `state_refcell_param` | STATE | violation | stops | ok | use of a disallowed type `std::cell::RefCell` |
| `state_static_atomic` | STATE | violation | stops | ok | [STATE] static item: only constants may be global, use `const` |
| `state_static_beside_allocator` | STATE | violation | stops | ok | [STATE] static item: only constants may be global, use `const` |
| `state_static_extern` | STATE | violation | stops | ok | [STATE] static item: only constants may be global, use `const` |
| `state_static_immutable` | STATE | violation | stops | ok | [STATE] static item: only constants may be global, use `const` |
| `state_static_in_function` | STATE | violation | stops | ok | [STATE] static item: only constants may be global, use `const` |
| `state_static_in_macro` | STATE | violation | stops | ok | [STATE] static item: only constants may be global, use `const` |
| `state_static_lifetime` | STATE | control | passes | ok |  |
| `state_static_mutex` | STATE | violation | stops | ok | [STATE] static item: only constants may be global, use `const` |
| `state_static_mut` | STATE | violation | stops | ok | [STATE] static item: only constants may be global, use `const` |
| `state_thread_local` | STATE | violation | stops | ok | [STATE] static item: only constants may be global, use `const` |
| `world_alias_chain` | WORLD | violation | stops | ok | [WORLD] `std::io::stdout` reaches the outside: obtain it in `main` and |
| `world_alias` | WORLD | violation | stops | ok | [WORLD] `std::env` reaches the outside: obtain it in `main` and pass i |
| `world_clock` | WORLD | violation | stops | ok | [WORLD] `std::time::Instant::now` reaches the outside: obtain it in `m |
| `world_dbg` | WORLD | violation | stops | ok | [WORLD] `dbg` reaches the outside: obtain it in `main` and pass it as  |
| `world_env_args` | WORLD | violation | stops | ok | [WORLD] `std::env::args` reaches the outside: obtain it in `main` and  |
| `world_env` | WORLD | violation | stops | ok | [WORLD] `std::env::var` reaches the outside: obtain it in `main` and p |
| `world_env_var_os` | WORLD | violation | stops | ok | [WORLD] `std::env::var_os` reaches the outside: obtain it in `main` an |
| `world_exit` | WORLD | violation | stops | ok | [WORLD] `std::process::exit` reaches the outside: obtain it in `main`  |
| `world_extern_crate_alias` | WORLD | violation | stops | ok | [WORLD] `std::env::var` reaches the outside: obtain it in `main` and p |
| `world_fs_read` | WORLD | violation | stops | ok | [WORLD] `std::fs::read_to_string` reaches the outside: obtain it in `m |
| `world_glob` | WORLD | violation | stops | ok | [WORLD] `std::env` reaches the outside: obtain it in `main` and pass i |
| `world_helper_of_main` | WORLD | violation | stops | ok | [WORLD] `std::env::var` reaches the outside: obtain it in `main` and p |
| `world_in_macro_argument` | WORLD | violation | stops | ok | [WORLD] `std::env::var` reaches the outside: obtain it in `main` and p |
| `world_in_macro_definition` | WORLD | violation | stops | ok | [WORLD] `std::env::var` reaches the outside: obtain it in `main` and p |
| `world_main_obtains` | WORLD | control | passes | ok |  |
| `world_path_exists` | WORLD | violation | stops | ok | use of a disallowed method `std::path::Path::exists` |
| `world_println` | WORLD | violation | stops | ok | [WORLD] `println` reaches the outside: obtain it in `main` and pass it |
| `world_public_main` | WORLD | violation | stops | ok | [WORLD] `std::env::var` reaches the outside: obtain it in `main` and p |
| `world_stdout_write` | WORLD | violation | stops | ok | [WORLD] `std::io::stdout` reaches the outside: obtain it in `main` and |
| `world_type_alias_unlisted_method` | WORLD | gap | passes | ok |  |
| `world_type_as_parameter` | WORLD | control | passes | ok |  |
| `world_use_function` | WORLD | violation | stops | ok | [WORLD] `std::fs::read` reaches the outside: obtain it in `main` and p |

Self-audit: ok. rust-harness: 3 file(s), 0 finding(s), 0 Clippy failure(s)
clippy 0.1.99 (58cf8f9c35 2026-10-13). Wrong verdicts: 0.
