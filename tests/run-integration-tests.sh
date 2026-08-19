#!/bin/sh

TEST_LOGDIR="${TEST_LOGDIR:-./integration/_logs}"
TEST_TIMEOUT="${TEST_TIMEOUT:-1}"
TEST_COMMAND="${TEST_COMMAND:-cargo run $TEST_CARGO_FLAGS --}"

rm -rf "$TEST_LOGDIR"
mkdir -p "$TEST_LOGDIR"

if $TEST_COMMAND --version
then
    :
else
    exit 1
fi

for testcasedir in ./integration/*/
do
    testcasename="$(basename "$testcasedir")"
    case "$testcasename" in
        _*) ;; # Ignore _infrastructure
        *)
            printf "Running integration test $testcasename ... "
            if timeout "$TEST_TIMEOUT" $TEST_COMMAND \
                       -c ./integration/_infrastructure \
                       -c "$testcasedir" \
                       --control \
                       > "$TEST_LOGDIR/${testcasename}-stdout" \
                       2> "$TEST_LOGDIR/${testcasename}-stderr"
            then
                printf "ok\n"
            else
                printf "FAIL $?\n"
            fi
            ;;
    esac
done
