# Congress Rolls

Congress rollcall data from the U.S. Senate and U.S. House of Representatives. 101 -> 118 Congresses (1989 - 2024).

## Overview

If you run the crate make sure you knwo what you're doing, most user interaction is done through the scripts in the `scripts` directory. Still, don't run things you don't understand.

XML and un-processed JSON data is available in the `full_data` directory, its compressed as `data_files.tar.gz`.

The master JSON file is compressed as `master_json.tar.gz` in the `full_data` directory.

The SQLite database is available as `votes.db` in the `full_data` directory.

## General Workflow

### Initial Setup

1. Download the data from the [full_data](full_data) directory.
2. Extract the data.
3. Run the `convert_all.sh` script to convert the XML data to JSON.
4. Run the `cargo run -- process_votes [json|sql]` to process the JSON data into a SQLite database or a master JSON file.

### Adding New Data

1. Download the new data using `cargo run -- download_xml [house|senate] [congress_number] [session_number] [rollcall_number]`.
2. Run `xml_to_json.sh [house|senate] [congress_number] [session_number] [job_count] [log_to_file]` to convert the XML data to JSON.
   This will try to convert the session data to JSON, existing data will be skipped.
3. Run `cargo run -- process_votes [json|sql]` to process the JSON data into a SQLite database or a master JSON file.
   If using the `sql` option, the existing database will be updated with the new data, if using the `json` option, you must reconstruct the master JSON file.
   The `sql` option also allows for the processing of individual JSON files, see the help message for more information.
   Overall, the `sql` option is the best option for adding, querying, and filtering data.


## Some Data Info

### sqlite database

The schema can be found in the `full_data` directory as `schema.sql`.

### json structure

> **WARNING**: This is a simplified version of the actual structure. The actual structure is much larger - 6GB+ of pure JSON data. Be prepared to handle large files.
> The SQLite database is a better option for querying and filtering data.
> The JSON structure is not optimized for querying, or really anything, its just a dump of the XML data.
> The SQLite database has significantly better performance and significantly more information (per vote/member/etc).

```json
{
  "chambers": {
    "house": {
      "congresses": {
        "110": {
          "sessions": {
            "2": {
              "rollcalls": [
                {
                  "rollcall_number": 525,
                  "vote_date": "24-Jul-2008",
                  "vote_question": "On Agreeing to the Resolution",
                  "vote_result": "Passed",
                  "vote_casts": [
                    {
                      "congress_number": 110,
                      "chamber": "house",
                      "session_number": 2,
                      "rollcall_number": 525,
                      "vote_date": "24-Jul-2008",
                      "vote_question": "On Agreeing to the Resolution",
                      "vote_result": "Passed",
                      "legislator_id": "A000014",
                      "legislator_name": "Abercrombie",
                      "party": "D",
                      "state": "HI",
                      "vote_cast": "Yea"
                    },
                    {
                      "congress_number": 110,
                      "chamber": "house",
                      "session_number": 2,
                      "rollcall_number": 525,
                      "vote_date": "24-Jul-2008",
                      "vote_question": "On Agreeing to the Resolution",
                      "vote_result": "Passed",
                      "legislator_id": "A000022",
                      "legislator_name": "Ackerman",
                      "party": "D",
                      "state": "NY",
                      "vote_cast": "Yea"
                    },
                    {
                      "congress_number": 110,
                      "chamber": "house",
                      "session_number": 2,
                      "rollcall_number": 525,
                      "vote_date": "24-Jul-2008",
                      "vote_question": "On Agreeing to the Resolution",
                      "vote_result": "Passed",
                      "legislator_id": "A000055",
                      "legislator_name": "Aderholt",
                      "party": "R",
                      "state": "AL",
                      "vote_cast": "Nay"
                    },
                    {
                      "congress_number": 110,
                      "chamber": "house",
                      "session_number": 2,
                      "rollcall_number": 525,
                      "vote_date": "24-Jul-2008",
                      "vote_question": "On Agreeing to the Resolution",
                      "vote_result": "Passed",
                      "legislator_id": "A000358",
                      "legislator_name": "Akin",
                      "party": "R",
                      "state": "MO",
                      "vote_cast": "Nay"
                    },
                    {
                      "congress_number": 110,
                      "chamber": "house",
                      "session_number": 2,
                      "rollcall_number": 525,
                      "vote_date": "24-Jul-2008",
                      "vote_question": "On Agreeing to the Resolution",
                      "vote_result": "Passed",
                      "legislator_id": "A000361",
                      "legislator_name": "Alexander",
                      "party": "R",
                      "state": "LA",
                      "vote_cast": "Nay"
                    },
                    {
                      "congress_number": 110,
                      "chamber": "house",
                      "session_number": 2,
                      "rollcall_number": 525,
                      "vote_date": "24-Jul-2008",
                      "vote_question": "On Agreeing to the Resolution",
                      "vote_result": "Passed",
                      "legislator_id": "A000357",
                      "legislator_name": "Allen",
                      "party": "D",
                      "state": "ME",
                      "vote_cast": "Yea"
                    },
                }
              ]
            }
          }
        }
      } 
    },
    "senate": {
        "...": "same structure as house",
    }
  }
}
```

## Other Projects

- [cdg_api](https://github.com/t-fbd/cdg_api) - Rust API wrapper for api.congress.gov
- [loc_api](https://github.com/t-fbd/loc_api) - Rust API wrapper for loc.gov


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

The data is sourced from the [U.S. Senate](https://www.senate.gov/legislative/votes) and [U.S. House of Representatives](https://clerk.house.gov/) websites.

## Contact

For questions or feedback, please contact me on [github](https://www.github.com/t-fbd) or email me [here](mailto:tairenfd@mailbox.org).

If you find this project helpful, consider donating [PayPal](https://paypal.me/imturn?country.x=US&locale.x=en_US).
