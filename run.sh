if [ "$#" -ne "2" ]; then
    echo "There are two main modes for the script - example and input mode"
    echo "Ex1: '$0 01a example'"
    echo "Ex2: '$0 24b input'"
    echo "Hint, only the first letter for the 2nd arg matters"
    exit 1
fi

mode="${2:0:1}"
prob="$1"

if [ $mode == "e" ]; then
    mode="example"
elif [ $mode == "i" ]; then
    mode="input"
else
    echo "Invalid mode \"$mode\" passed"
    exit 1
fi

num=${prob:0:2}

# Will evaluate something similar to this:
# < data/01_example.txt cargo run --bin p01b src/bin/p01b.rs
< data/${num}_${mode}.txt cargo run --bin p${prob} src/bin/p${prob}.rs
