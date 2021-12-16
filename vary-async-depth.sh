#!/usr/bin/env bash
set -euo pipefail
shopt -s extglob

echo "# Varying number of async functions called

| # of async fns | Item                                             | Self time | % of total time | Time     | Item count | Incremental result hashing time |
|---|--------------------------------------------------|-----------|-----------------|----------|------------|---------------------------------|" > vary-async-depth-results.md

for MAX_DEPTH in {1..30}
do

    echo "pub async fn entry_point() {
    let x = X$MAX_DEPTH::new();
    x.f().await;
}

async fn term_async() -> u32 {
    1
}

fn term_sync() -> u32 {
    1
}

struct X0 {}
impl X0 {
    fn new() -> Self {
        Self {}
    }

    async fn f(&self) -> u32 {
        term_sync() + term_async().await
    }
}" > helper_crate/src/lib.rs

    for DEPTH in $( seq 1 $MAX_DEPTH )
    do
        echo "struct X$DEPTH {
    x: X$(($DEPTH - 1)),
}
impl X$DEPTH {
    fn new() -> Self {
        Self {
            x: X$(($DEPTH - 1))::new(),
        }
    }

    async fn f(&self) -> u32 {
        term_sync() + self.x.f().await
    }
}" >> helper_crate/src/lib.rs

    done

    RESULTS=$(./test.sh)

    echo "| $MAX_DEPTH $RESULTS" >> vary-async-depth-results.md

done

git checkout helper_crate/src/lib.rs
