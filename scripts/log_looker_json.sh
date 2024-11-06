#!/bin/bash

# log_looker_json.sh
# Description: summarize and analyze the output when constructing a JSON file

# Function to display usage instructions
usage() {
    echo "Usage: $0 <log_file>"
    echo "Example: $0 application.log"
    exit 1
}

# Check if exactly one argument (log file) is provided
if [ "$#" -ne 1 ]; then
    echo "❌ Error: Incorrect number of arguments."
    usage
fi

LOG_FILE="$1"

# Check if the file exists and is readable
if [ ! -f "$LOG_FILE" ]; then
    echo "❌ Error: File '$LOG_FILE' does not exist."
    exit 1
fi

if [ ! -r "$LOG_FILE" ]; then
    echo "❌ Error: File '$LOG_FILE' is not readable."
    exit 1
fi

echo "📄 Analyzing log file: $LOG_FILE"

# Initialize counters
START_COUNT=0
COMPLETION_COUNT=0
DIR_CREATION_COUNT=0
WARNING_COUNT=0
ERROR_COUNT=0
INFO_COUNT=0

# Temporary files to store detailed entries (optional)
START_DETAILS=$(mktemp)
COMPLETION_DETAILS=$(mktemp)
DIR_CREATION_DETAILS=$(mktemp)
WARNING_DETAILS=$(mktemp)
ERROR_DETAILS=$(mktemp)
INFO_DETAILS=$(mktemp)

# Parse the log file
while IFS= read -r line
do
    if [[ "$line" == *"📥 Starting"* ]]; then
        ((START_COUNT++))
        echo "$line" >> "$START_DETAILS"
    elif [[ "$line" == *"✅"* && "$line" != *"✅ Inserted"* ]]; then
        # Exclude "✅ Inserted" if it's counted separately
        ((COMPLETION_COUNT++))
        echo "$line" >> "$COMPLETION_DETAILS"
    elif [[ "$line" == *"✅ Inserted"* ]]; then
        ((COMPLETION_COUNT++))
        echo "$line" >> "$COMPLETION_DETAILS"
    elif [[ "$line" == *"📁 Created directories"* ]]; then
        ((DIR_CREATION_COUNT++))
        echo "$line" >> "$DIR_CREATION_DETAILS"
    elif [[ "$line" == *"⚠️"* ]]; then
        ((WARNING_COUNT++))
        echo "$line" >> "$WARNING_DETAILS"
    elif [[ "$line" == *"❌"* ]]; then
        ((ERROR_COUNT++))
        echo "$line" >> "$ERROR_DETAILS"
    elif [[ "$line" == *"📄"* || "$line" == *"🛑"* || "$line" == *"🎉"* ]]; then
        ((INFO_COUNT++))
        echo "$line" >> "$INFO_DETAILS"
    fi
done < "$LOG_FILE"

# Display the summary
echo "----------------------------------------"
echo "📊 Log Summary:"
echo "📥 Task Starts       : $START_COUNT"
echo "✅ Task Completions  : $COMPLETION_COUNT"
echo "📁 Directory Creations: $DIR_CREATION_COUNT"
echo "⚠️ Warnings          : $WARNING_COUNT"
echo "❌ Critical Errors    : $ERROR_COUNT"
echo "📄 Informational Messages: $INFO_COUNT"
echo "----------------------------------------"

# Optionally display detailed entries

# Function to display details if count > 0
display_details() {
    local count=$1
    local title=$2
    local file=$3
    if [ "$count" -gt 0 ]; then
        echo ""
        echo "$title:"
        cat "$file"
    fi
}

display_details "$START_COUNT" "📥 Task Start Entries" "$START_DETAILS"
display_details "$COMPLETION_COUNT" "✅ Task Completion Entries" "$COMPLETION_DETAILS"
display_details "$DIR_CREATION_COUNT" "📁 Directory Creation Entries" "$DIR_CREATION_DETAILS"
display_details "$WARNING_COUNT" "⚠️ Warning Entries" "$WARNING_DETAILS"
display_details "$ERROR_COUNT" "❌ Critical Error Entries" "$ERROR_DETAILS"
display_details "$INFO_COUNT" "📄 Informational Entries" "$INFO_DETAILS"

# Clean up temporary files
rm "$START_DETAILS" "$COMPLETION_DETAILS" "$DIR_CREATION_DETAILS" "$WARNING_DETAILS" "$ERROR_DETAILS" "$INFO_DETAILS"

echo "✅ Log analysis complete."

exit 0
