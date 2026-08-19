#!/bin/sh

specifictestdir="$1"
if [ -n "$specifictestdir" ]; then
    shift
fi
if [ "$1" = "--fg" ]; then
    shift
    foreground=yes
else
    foreground=
fi

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

runtest() {
    testcasedir="$1"
    testcasename="$(basename "$testcasedir")"
    printf "Running integration test $testcasename ... "
    if
        if [ -n "$foreground" ]
        then
            $TEST_COMMAND \
                -c ./integration/_infrastructure \
                -c "$testcasedir" \
                --control
        else
            timeout "$TEST_TIMEOUT" $TEST_COMMAND \
                    -c ./integration/_infrastructure \
                    -c "$testcasedir" \
                    --control \
                    > "$TEST_LOGDIR/${testcasename}-stdout" \
                    2> "$TEST_LOGDIR/${testcasename}-stderr"
        fi
    then
        printf "ok\n"
    else
        printf "FAIL $?\n"
    fi
}

if [ -n "$specifictestdir" ]
then
    runtest "$specifictestdir"
else
    for testcasedir in ./integration/*/
    do
        testcasename="$(basename "$testcasedir")"
        case "$testcasename" in
            _*)
            # Ignore _infrastructure
            ;;
            *)
                runtest "$testcasedir"
                ;;
        esac
    done
fi