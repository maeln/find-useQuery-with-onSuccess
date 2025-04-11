# What / Why

React Query v5 decided to remove all the `onError`, `onSuccess` and `onSettled` callback from `useQuery` (for good reason).
See the changelog: https://tanstack.com/query/latest/docs/framework/react/guides/migrating-to-v5#callbacks-on-usequery-and-queryobserver-have-been-removed

So to migrate easily from previous version to the v5, you need to find and remove all those callback from useQuery calls.

# Usage

```
typescript-usequery-check <path to a ts or tsx file>
```

# How to use with [fd](https://github.com/sharkdp/fd) to find all the occurence in your project

First install `fd`.
Then build this:

```
cargo build --release
```

Put the binary (`target/release/typescript-usequery-checker`) somewhere in your `$PATH`.
Then call it with fd at the root of your project:
```
fd -e ts -e tsx -x typescript-usequery-checker
```
