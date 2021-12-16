#!/usr/bin/env bash
set -euo pipefail
shopt -s extglob

echo "# Varying number of params/lifetimes

| # of params/lifetimes | Item                                             | Self time | % of total time | Time     | Item count | Incremental result hashing time |" > vary-params-lifetimes-results.md

for NUM_PARAMS in {1..30}
do
    echo "fn main() {}

    trait Trait {
    #[must_use]
    fn f<
        'life0," > repro_crate/src/main.rs

    for P in $( seq 1 $NUM_PARAMS )
    do
        echo "        'life$P," >> repro_crate/src/main.rs
    done

    echo "        'async_trait,
    >(
        &'life0 self," >> repro_crate/src/main.rs

    for P in $( seq 1 $NUM_PARAMS )
    do
        echo "        p$P: &'life$P str," >> repro_crate/src/main.rs
    done

    echo "    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait," >> repro_crate/src/main.rs

    for P in $( seq 1 $NUM_PARAMS )
    do
        echo "        'life$P: 'async_trait," >> repro_crate/src/main.rs
    done

    echo "        Self: 'async_trait;
}" >> repro_crate/src/main.rs

    for NUM_IMPLS in {0..30}
    do
        echo "pub struct S$NUM_IMPLS;
    impl Trait for S$NUM_IMPLS {
        fn f<
            'life0," >> repro_crate/src/main.rs

        for P in $( seq 1 $NUM_PARAMS )
        do
            echo "        'life$P," >> repro_crate/src/main.rs
        done

        echo "        'async_trait,
        >(
            &'life0 self," >> repro_crate/src/main.rs

        for P in $( seq 1 $NUM_PARAMS )
        do
            echo "        _p$P: &'life$P str," >> repro_crate/src/main.rs
        done

        echo "    ) -> ::core::pin::Pin<
            Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
        >
        where
            'life0: 'async_trait," >> repro_crate/src/main.rs

        for P in $( seq 1 $NUM_PARAMS )
        do
            echo "        'life$P: 'async_trait," >> repro_crate/src/main.rs
        done

        echo "        Self: 'async_trait,
        {
            Box::pin(async move {
                let __self = self;" >> repro_crate/src/main.rs

        for P in $( seq 1 $NUM_PARAMS )
        do
            echo "            let _p$P = _p$P;" >> repro_crate/src/main.rs
        done

        echo "            let _: () = {
                    helper_crate::entry_point().await;
                };
            })
        }
    }" >> repro_crate/src/main.rs
    done

RESULTS=$(./test.sh)

echo "| $NUM_PARAMS $RESULTS" >> vary-params-lifetimes-results.md

done

git checkout repro_crate/src/main.rs
