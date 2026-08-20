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

TEST_LOGDIR="${TEST_LOGDIR:-./tests/integration/_logs}"
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

finalstatus=0

runtest() {
    testcasedir="$1"
    testcasename="$(basename "$testcasedir")"
    printf "Running integration test $testcasename ... "
    if
        if [ -n "$foreground" ]
        then
            $TEST_COMMAND \
                -c ./tests/integration/_infrastructure \
                -c "$testcasedir" \
                --control
        else
            timeout "$TEST_TIMEOUT" $TEST_COMMAND \
                    -c ./tests/integration/_infrastructure \
                    -c "$testcasedir" \
                    --control \
                    > "$TEST_LOGDIR/${testcasename}-stdout" \
                    2> "$TEST_LOGDIR/${testcasename}-stderr"
        fi
    then
        printf "ok\n"
    else
        printf "FAIL $?\n"
        finalstatus=1
    fi
}

if [ -n "$specifictestdir" ]
then
    runtest "$specifictestdir"
else
    for testcasedir in ./tests/integration/*/
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

if [ "$finalstatus" = "0" ]
then
    echo 'All tests passed!'
fi
exit $finalstatus
