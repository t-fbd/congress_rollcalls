#!/bin/bash

# =============================================================================
# Script: xml_to_json.sh
# Description: Converts XML files to JSON using yq or yq-go based on provided 
#              Chamber, Congress Number, Session Number, and logging preference.
# Usage: ./xml_to_json.sh <chamber> <congress_number> <session_number> <job_count> <log_to_file>
# Example: ./xml_to_json.sh house 117 1 4 true
# =============================================================================

# === Function Definitions ===

# Function to display usage information
usage() {
    echo "Usage: $0 <chamber> <congress_number> <session_number> <job_count> <log_to_file>"
    echo "  <chamber>         : 'house' or 'senate'"
    echo "  <congress_number> : Integer representing the Congress number (e.g., 117)"
    echo "  <session_number>  : Integer representing the Session number (e.g., 1)"
    echo "  <job_count>       : Number of parallel jobs to run (e.g., 4)"
    echo "  <log_to_file>     : 'true' to log output to a file, 'false' to log to stdout"
    echo "Example:"
    echo "  $0 house 117 1 4 true"
    exit 1
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# === Pre-flight Checks ===

# Check if the correct number of arguments is provided
if [ "$#" -ne 5 ]; then
    echo "Error: Invalid number of arguments."
    usage
fi

# Assign arguments to variables
CHAMBER="$1"
CONGRESS_NUMBER="$2"
SESSION_NUMBER="$3"
PARALLEL_JOBS="$4"
LOG_TO_FILE="$5"

# Validate Congress Number (should be a positive integer >= 101)
if ! [[ "$CONGRESS_NUMBER" =~ ^[0-9]+$ ]] || [ "$CONGRESS_NUMBER" -lt 101 ]; then
    echo "Error: Congress number must be a positive integer, 101 or greater."
    usage
fi

# Validate Chamber (should be 'house' or 'senate')
if [[ "$CHAMBER" != "house" && "$CHAMBER" != "senate" ]]; then
    echo "Error: Chamber must be either 'house' or 'senate'."
    usage
fi

# Validate Session Number (should be either 1 or 2)
if ! [[ "$SESSION_NUMBER" =~ ^[0-9]+$ ]] || [ "$SESSION_NUMBER" -lt 1 ] || [ "$SESSION_NUMBER" -gt 2 ]; then
    echo "Error: Session number must be a positive integer (1 or 2)."
    usage
fi

# Validate Parallel Jobs (should be a positive integer)
if ! [[ "$PARALLEL_JOBS" =~ ^[0-9]+$ ]] || [ "$PARALLEL_JOBS" -lt 1 ]; then
    echo "Error: Job count must be a positive integer."
    usage
fi

# Validate Log To File (should be 'true' or 'false')
if [[ "$LOG_TO_FILE" != "true" && "$LOG_TO_FILE" != "false" ]]; then
    echo "Error: log_to_file must be either 'true' or 'false'."
    usage
fi

# Check if yq or yq-go is installed
if command_exists yq-go; then
    YQ_CMD="yq-go"
elif command_exists yq; then
    YQ_CMD="yq"
else
    echo "Error: Neither yq-go nor yq is installed or not in PATH."
    echo "Please install yq or yq-go before running this script."
    exit 1
fi

# === Define Directory Paths ===

# Determine the project root directory (assuming script is in scripts/)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Base directories
BASE_XML_DIR="$PROJECT_ROOT/data/xml"
BASE_JSON_DIR="$PROJECT_ROOT/data/json"
BASE_LOG_DIR="$PROJECT_ROOT/scripts/logs"

# Construct input and output directories based on arguments
INPUT_DIR="$BASE_XML_DIR/$CONGRESS_NUMBER/$CHAMBER/$SESSION_NUMBER"
OUTPUT_DIR="$BASE_JSON_DIR/$CONGRESS_NUMBER/$CHAMBER/$SESSION_NUMBER"

# Check if INPUT_DIR exists
if [ ! -d "$INPUT_DIR" ]; then
    echo "Error: Input directory '$INPUT_DIR' does not exist."
    exit 1
fi

# Create OUTPUT_DIR if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# === Define Logging ===

if [ "$LOG_TO_FILE" = "true" ]; then
    # Ensure the log directory exists
    mkdir -p "$BASE_LOG_DIR"
    
    # Define a unique log file name based on parameters and timestamp
    TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
    LOG_FILE="$BASE_LOG_DIR/conversion_${CHAMBER}_${CONGRESS_NUMBER}_${SESSION_NUMBER}_$TIMESTAMP.log"
    
    # Initialize the log file
    > "$LOG_FILE"
    
    # Redirect all stdout and stderr to the log file
    exec > >(tee -a "$LOG_FILE") 2>&1
fi

# === Processing with Parallel Jobs ===

echo "Starting conversion process..."
echo "Congress Number : $CONGRESS_NUMBER"
echo "Chamber         : $CHAMBER"
echo "Session Number  : $SESSION_NUMBER"
echo "Input Directory : $INPUT_DIR"
echo "Output Directory: $OUTPUT_DIR"
echo "Parallel Jobs   : $PARALLEL_JOBS"
echo "Log to File     : $LOG_TO_FILE"
echo "----------------------------------------"

# Export variables needed in subshells
export YQ_CMD
export INPUT_DIR
export OUTPUT_DIR

# Find all .xml files in INPUT_DIR and process them in parallel
find "$INPUT_DIR" -type f -name "*.xml" -print0 | \
xargs -0 -n1 -P "$PARALLEL_JOBS" bash -c '
    xml_file="$0"
    rel_path="${xml_file#'"$INPUT_DIR"'/}"
    filename=$(basename "$rel_path" .xml)
    json_file="'"$OUTPUT_DIR"'/$filename.json"

    # If JSON file already exists, skip conversion
    if [ -f "$json_file" ]; then
        echo "⏭️ Skipping: $(basename "$xml_file") → $(basename "$json_file")"
        exit 0
    fi

    # Convert XML to JSON
    if '"$YQ_CMD"' -P -o=json "$xml_file" | sed "s/[@+]//g" > "$json_file"; then
        echo "✅ Converted: $(basename "$xml_file") → $(basename "$json_file")"
    else
        echo "❌ Failed to convert: $(basename "$xml_file")" >&2
    fi
' 

echo "----------------------------------------"
echo "🎉 Conversion process completed."
