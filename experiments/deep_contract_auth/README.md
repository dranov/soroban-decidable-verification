# Experiments with authorization

Experiments with Soroban's authorization framework, particularly to understand
how nested authorization works.

These experiments were prompted by this paragraph in
[CAP 0046-11](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0046-11.md),
which apparently suggests that ordering in authorization entries is not strictly
enforced:

> It is possible to have multiple authorized trees for the same address with the
> root in the same stack frame. In such case the inner nodes can be interchanged
> between trees while still satisfying the algorithm. For example, if contract A
> calls require_auth twice, then calls B and C both of which call require_auth,
> the following combinations of SorobanAuthorizationEntry invocations will pass
> authorization algorithm: A->[B, C], A, A->B, A->C, A->C, A->B, A, A->[B, C].
> Note, that sequencing the calls changes the requirements, for example if A
> calls require_auth right before calling B and C, only the following
> combinations would pass: A->B, A->C, A->[B, C], A.

## Observations

- direct contract calls (e.g. `A` calls `B`) are always authorized, i.e. a call
  to `A.require_auth()` in B will always succeed;

- if `A` authorizes `B` to call contract `C` on `A`'s behalf, this authorization
  seems to hold regardless of `B`'s call stack down to `C`, i.e. there can be
  any number of intermediary calls, as long as they don't "consume" the
  authorization; (see `test_indirect_call`)

- authorizations passed by `authorize_as_current_contract` only apply to the ONE
  following contract call (rather than all calls made in the current function),
  i.e. only to the first child of the call stack that starts from the current
  function; (see `test_indirect_double_call_single_auth` and
  `test_indirect_double_call`)

- if `A` wants to authorize `B` to call `C` twice, it must pass two root
  authorizations in the `authorize_as_current_contract` call; (see
  `test_double_authorization`)

- if the root of the authorization does not match, then sub-trees will not match
  either (so you can't "split" an authorization; see
  [Discord](https://discord.com/channels/897514728459468821/1149701029387055166/1152712204118929548));
  (see `test_sub_invocation_bad_root`);
    - see also `test_sub_invocation_good_root`
    - **TODO**: write a test to ensure this holds for arbitrarily deep call stacks