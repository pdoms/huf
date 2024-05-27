#!/usr/bin/sh

mode=$1
out=$2


if [[ "$mode" = "build" ]];then
    if [ -z "$out" ]; then
        rustc main.rs -o huf
    else
        rustc main.rs -O -o $out
    fi
elif [[ "$mode" = "test" ]];then
    if [ -z "$out" ]; then
        rustc --test main.rs -o huf_test
    else
        rustc --test main.rs -o $out
    fi
elif [[ "$mode" = "itest" ]];then
    if [ -z "$out" ]; then
        "$out wizard_of_oz.txt" 
        
    fi
elif [[ "$mode" = "clean" ]];then
    if [ -f ./test_input.huf ]; then
        rm test_input.huf
    fi
    if [ -f ./huf ]; then
        rm huf
    fi
    if [ -f ./huf_test ]; then
        rm huf_test 
    fi
else
    echo "unknown command '$mode' -> known commands are: 'build', 'test', 'clean', 'itest'"
    exit 1
fi

