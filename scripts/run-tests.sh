#!/bin/bash
# Comprehensive testing script

set -e

echo "🧪 Running Stellar Launchpad Test Suite..."
echo "========================================"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track test results
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

run_test_suite() {
    local name="$1"
    local path="$2"
    
    echo -e "\n📋 Testing: ${YELLOW}$name${NC}"
    echo "----------------------------------------"
    
    cd "$path"
    
    if cargo test --quiet; then
        local test_count=$(cargo test 2>&1 | grep "test result:" | head -1 | grep -o '[0-9]* passed' | cut -d' ' -f1)
        echo -e "${GREEN}✅ $name: $test_count tests passed${NC}"
        PASSED_TESTS=$((PASSED_TESTS + test_count))
        TOTAL_TESTS=$((TOTAL_TESTS + test_count))
    else
        echo -e "${RED}❌ $name: Tests failed${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        cd - > /dev/null
        return 1
    fi
    
    cd - > /dev/null
    return 0
}

# Run all test suites
echo "Starting test execution..."

run_test_suite "Token Contract" "contracts/token"
run_test_suite "Vesting Contract" "contracts/vesting"  
run_test_suite "Airdrop Contract" "contracts/airdrop"
run_test_suite "Launchpad Registry" "contracts/launchpad"
run_test_suite "CLI Tool" "crates/cli"

# Summary
echo -e "\n🏁 Test Summary"
echo "========================================"
echo -e "Total Tests: ${YELLOW}$TOTAL_TESTS${NC}"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "\n${GREEN}🎉 All tests passed! Great work!${NC}"
    exit 0
else
    echo -e "\n${RED}💥 Some tests failed. Please check the output above.${NC}"
    exit 1
fi