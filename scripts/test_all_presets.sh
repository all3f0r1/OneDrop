#!/bin/bash
# Test all presets in test-presets-200/

set -e

PRESET_DIR="/home/ubuntu/OneDrop/test-presets-200"
RESULTS_FILE="/tmp/preset_test_results.txt"
SUCCESS_COUNT=0
FAIL_COUNT=0

echo "=== OneDrop Preset Testing ===" > "$RESULTS_FILE"
echo "Date: $(date)" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Count total presets
TOTAL=$(find "$PRESET_DIR" -name "*.milk" | wc -l)
echo "Total presets to test: $TOTAL"
echo "Total presets: $TOTAL" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# Test each preset
for preset in "$PRESET_DIR"/*.milk; do
    BASENAME=$(basename "$preset")
    echo -n "Testing $BASENAME... "
    
    if cargo run --release -p onedrop-cli -- info "$preset" > /dev/null 2>&1; then
        echo "✓ OK"
        echo "✓ $BASENAME" >> "$RESULTS_FILE"
        ((SUCCESS_COUNT++))
    else
        echo "✗ FAIL"
        echo "✗ $BASENAME" >> "$RESULTS_FILE"
        ((FAIL_COUNT++))
    fi
done

echo "" >> "$RESULTS_FILE"
echo "=== Summary ===" >> "$RESULTS_FILE"
echo "Success: $SUCCESS_COUNT / $TOTAL" >> "$RESULTS_FILE"
echo "Failed: $FAIL_COUNT / $TOTAL" >> "$RESULTS_FILE"
echo "Success rate: $(echo "scale=2; $SUCCESS_COUNT * 100 / $TOTAL" | bc)%" >> "$RESULTS_FILE"

echo ""
echo "=== Test Complete ==="
echo "Success: $SUCCESS_COUNT / $TOTAL"
echo "Failed: $FAIL_COUNT / $TOTAL"
echo "Success rate: $(echo "scale=2; $SUCCESS_COUNT * 100 / $TOTAL" | bc)%"
echo ""
echo "Full results saved to: $RESULTS_FILE"

cat "$RESULTS_FILE"
