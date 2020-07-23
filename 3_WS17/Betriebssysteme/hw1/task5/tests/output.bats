#!/usr/bin/env bats


@test "Check that we have a debug output" {
    run stat "$BATS_TEST_DIRNAME/../target/debug/task5"
    [ "$status" -eq 0 ]
}

@test "Output must be from 1..30 and correct formated" {
    run "$BATS_TEST_DIRNAME/../target/debug/task5"
    [[ "${lines[0]}" =~ "1" ]]
    [[ "${lines[1]}" =~ "2*" ]]
    [[ "${lines[2]}" =~ "3*" ]]
    [[ "${lines[3]}" =~ "4" ]]
    [[ "${lines[4]}" =~ "5" ]]
    [[ "${lines[25]}" =~ "26" ]]
    [[ "${lines[26]}" =~ "27" ]]
    [[ "${lines[27]}" =~ "28" ]]
    [[ "${lines[28]}" =~ "29*" ]]
    [[ "${lines[29]}" =~ "30" ]]
}

# wc output with white spaces is trimmed by xargs
@test "Output must be exact 30 lines long" {
    run bash -c "'$BATS_TEST_DIRNAME/../target/debug/task5' | wc -l | xargs"
    [ "$output" = "30" ]
}
