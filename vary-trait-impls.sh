#!/usr/bin/env bash
set -euo pipefail
shopt -s extglob

echo "fn main() {}
trait Trait {
    #[must_use]
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait;
}" > repro_crate/src/main.rs

echo "# Varying number of trait implementations

| # of trait impls | Item                                             | Self time | % of total time | Time     | Item count | Incremental result hashing time |" > vary-trait-impls-results.md

for NUM_IMPLS in {0..30}
do
    echo "pub struct S$NUM_IMPLS;
impl Trait for S$NUM_IMPLS {
    fn f<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let __self = self;
            let _: () = {
                helper_crate::entry_point().await;
            };
        })
    }
}" >> repro_crate/src/main.rs

RESULTS=$(./test.sh)

echo "| $NUM_IMPLS $RESULTS" >> vary-trait-impls-results.md

done

git checkout repro_crate/src/main.rs
