#! /bin/sh

# =============================================================================
# run this within the scripts directory
#
# Usage: ./convert_all.sh
#
# the provided arguments to the xml_to_json.sh script are:
# chamber congress session job_count log_to_file
#
# both this and the xml_to_json.sh script are dependent on the xml_files being
# activated via ../src/main.rs in the main() function
#
# chamber: house or senate
# congress: 101 to 118
# session: 1 or 2
# job_count: check with nproc
# log_to_file: true or false
# =============================================================================

# house
../scripts/xml_to_json.sh house 116 1 16 true
../scripts/xml_to_json.sh house 116 2 16 true
../scripts/xml_to_json.sh house 115 1 16 true
../scripts/xml_to_json.sh house 115 2 16 true
../scripts/xml_to_json.sh house 114 1 16 true
../scripts/xml_to_json.sh house 114 2 16 true
../scripts/xml_to_json.sh house 113 1 16 true
../scripts/xml_to_json.sh house 113 2 16 true
../scripts/xml_to_json.sh house 112 1 16 true
../scripts/xml_to_json.sh house 112 2 16 true
../scripts/xml_to_json.sh house 111 1 16 true
../scripts/xml_to_json.sh house 111 2 16 true
../scripts/xml_to_json.sh house 110 1 16 true
../scripts/xml_to_json.sh house 110 2 16 true
../scripts/xml_to_json.sh house 109 1 16 true
../scripts/xml_to_json.sh house 109 2 16 true
../scripts/xml_to_json.sh house 108 1 16 true
../scripts/xml_to_json.sh house 108 2 16 true
../scripts/xml_to_json.sh house 107 1 16 true
../scripts/xml_to_json.sh house 107 2 16 true
../scripts/xml_to_json.sh house 106 1 16 true
../scripts/xml_to_json.sh house 106 2 16 true
../scripts/xml_to_json.sh house 105 1 16 true
../scripts/xml_to_json.sh house 105 2 16 true
../scripts/xml_to_json.sh house 104 1 16 true
../scripts/xml_to_json.sh house 104 2 16 true
../scripts/xml_to_json.sh house 103 1 16 true
../scripts/xml_to_json.sh house 103 2 16 true
../scripts/xml_to_json.sh house 102 1 16 true
../scripts/xml_to_json.sh house 102 2 16 true
../scripts/xml_to_json.sh house 101 1 16 true
../scripts/xml_to_json.sh house 101 2 16 true

# senate
../scripts/xml_to_json.sh senate 116 1 16 true
../scripts/xml_to_json.sh senate 116 2 16 true
../scripts/xml_to_json.sh senate 115 1 16 true
../scripts/xml_to_json.sh senate 115 2 16 true
../scripts/xml_to_json.sh senate 114 1 16 true
../scripts/xml_to_json.sh senate 114 2 16 true
../scripts/xml_to_json.sh senate 113 1 16 true
../scripts/xml_to_json.sh senate 113 2 16 true
../scripts/xml_to_json.sh senate 112 1 16 true
../scripts/xml_to_json.sh senate 112 2 16 true
../scripts/xml_to_json.sh senate 111 1 16 true
../scripts/xml_to_json.sh senate 111 2 16 true
../scripts/xml_to_json.sh senate 110 1 16 true
../scripts/xml_to_json.sh senate 110 2 16 true
../scripts/xml_to_json.sh senate 109 1 16 true
../scripts/xml_to_json.sh senate 109 2 16 true
../scripts/xml_to_json.sh senate 108 1 16 true
../scripts/xml_to_json.sh senate 108 2 16 true
../scripts/xml_to_json.sh senate 107 1 16 true
../scripts/xml_to_json.sh senate 107 2 16 true
../scripts/xml_to_json.sh senate 106 1 16 true
../scripts/xml_to_json.sh senate 106 2 16 true
../scripts/xml_to_json.sh senate 105 1 16 true
../scripts/xml_to_json.sh senate 105 2 16 true
../scripts/xml_to_json.sh senate 104 1 16 true
../scripts/xml_to_json.sh senate 104 2 16 true
../scripts/xml_to_json.sh senate 103 1 16 true
../scripts/xml_to_json.sh senate 103 2 16 true
../scripts/xml_to_json.sh senate 102 1 16 true
../scripts/xml_to_json.sh senate 102 2 16 true
../scripts/xml_to_json.sh senate 101 1 16 true
../scripts/xml_to_json.sh senate 101 2 16 true
