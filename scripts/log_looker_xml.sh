#!/bin/bash

# log_looker_xml.sh
# Description: Analyzes log files generated by the XML download process and provides a detailed summary.

# Function to display usage instructions
usage() {
    echo "Usage: $0 <log_file>"
    echo "Example: $0 xml_download.log"
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

echo "📄 Analyzing XML download log file: $LOG_FILE"

# Initialize counters
START_COUNT=0
COMPLETION_COUNT=0
DIR_CREATION_COUNT=0
FILE_PROCESS_COUNT=0
WARNING_COUNT=0
POSSIBLE_ERROR_COUNT=0
CRITICAL_ERROR_COUNT=0
INFO_COUNT=0

# Temporary files to store detailed entries (optional)
START_DETAILS=$(mktemp)
COMPLETION_DETAILS=$(mktemp)
DIR_CREATION_DETAILS=$(mktemp)
FILE_PROCESS_DETAILS=$(mktemp)
WARNING_DETAILS=$(mktemp)
POSSIBLE_ERROR_DETAILS=$(mktemp)
CRITICAL_ERROR_DETAILS=$(mktemp)
INFO_DETAILS=$(mktemp)

# Parse the log file
while IFS= read -r line
do
    if [[ "$line" == *"📥 Starting XML download"* ]]; then
        ((START_COUNT++))
        echo "$line" >> "$START_DETAILS"
    elif [[ "$line" == *"✅ XML download completed"* ]]; then
        ((COMPLETION_COUNT++))
        echo "$line" >> "$COMPLETION_DETAILS"
    elif [[ "$line" == *"📁 Created directories"* ]]; then
        ((DIR_CREATION_COUNT++))
        echo "$line" >> "$DIR_CREATION_DETAILS"
    elif [[ "$line" == *"📄 Processing file"* ]]; then
        ((FILE_PROCESS_COUNT++))
        echo "$line" >> "$FILE_PROCESS_DETAILS"
    elif [[ "$line" == *"⚠️"* ]]; then
        ((WARNING_COUNT++))
        echo "$line" >> "$WARNING_DETAILS"
    elif [[ "$line" == *"🛑"* ]]; then
        ((POSSIBLE_ERROR_COUNT++))
        echo "$line" >> "$POSSIBLE_ERROR_DETAILS"
    elif [[ "$line" == *"❌"* ]]; then
        ((CRITICAL_ERROR_COUNT++))
        echo "$line" >> "$CRITICAL_ERROR_DETAILS"
    elif [[ "$line" == *"🎉"* || "$line" == *"📄"* || "$line" == *"🛑"* ]]; then
        ((INFO_COUNT++))
        echo "$line" >> "$INFO_DETAILS"
    fi
done < "$LOG_FILE"

# Display the summary
echo "----------------------------------------"
echo "📊 XML Download Log Summary:"
echo "📥 Download Starts          : $START_COUNT"
echo "✅ Download Completions     : $COMPLETION_COUNT"
echo "📁 Directory Creations      : $DIR_CREATION_COUNT"
echo "📄 File Processing Events   : $FILE_PROCESS_COUNT"
echo "⚠️ Warnings                 : $WARNING_COUNT"
echo "🛑 Possible Errors          : $POSSIBLE_ERROR_COUNT"
echo "❌ Critical Errors           : $CRITICAL_ERROR_COUNT"
echo "🎉 Informational Messages   : $INFO_COUNT"
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

display_details "$START_COUNT" "📥 Download Start Entries" "$START_DETAILS"
display_details "$COMPLETION_COUNT" "✅ Download Completion Entries" "$COMPLETION_DETAILS"
display_details "$DIR_CREATION_COUNT" "📁 Directory Creation Entries" "$DIR_CREATION_DETAILS"
display_details "$FILE_PROCESS_COUNT" "📄 File Processing Entries" "$FILE_PROCESS_DETAILS"
display_details "$WARNING_COUNT" "⚠️ Warning Entries" "$WARNING_DETAILS"
display_details "$POSSIBLE_ERROR_COUNT" "🛑 Possible Error Entries" "$POSSIBLE_ERROR_DETAILS"
display_details "$CRITICAL_ERROR_COUNT" "❌ Critical Error Entries" "$CRITICAL_ERROR_DETAILS"
display_details "$INFO_COUNT" "🎉 Informational Entries" "$INFO_DETAILS"

# Clean up temporary files
rm "$START_DETAILS" "$COMPLETION_DETAILS" "$DIR_CREATION_DETAILS" "$FILE_PROCESS_DETAILS" \
   "$WARNING_DETAILS" "$POSSIBLE_ERROR_DETAILS" "$CRITICAL_ERROR_DETAILS" "$INFO_DETAILS"

echo "✅ XML download log analysis complete."

exit 0