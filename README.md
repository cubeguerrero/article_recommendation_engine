# Article Recommendation Engine

## What is this
I'm trying to learn Rust and basics of Data Engineering. I asked Gemini to help generate a simple project plan to help me learn.

### Gemini's Plan
```
As a principal data engineer at a FAANG company, I've seen many junior engineers successfully transition into strong data engineers. It's fantastic that you're looking to dive into both data engineering and Rust – that's a powerful combination with a lot of potential!

For a beginner project, the key is to touch upon the core pillars of data engineering without getting bogged down in excessive complexity. You want something that teaches you about:

Ingestion: Getting data from a source.
Transformation: Cleaning, enriching, and shaping the data.
Storage: Persisting the data in a usable format.
Orchestration (optional but good to think about): Automating the process.
Rust Integration: Using Rust for performance-critical or data-intensive tasks.
Here's a simple, yet comprehensive, beginner data engineering project idea that incorporates Rust:

Project Recommendation: "Personalized Article Recommendation Engine (Simplified)"

This project simulates a very basic article recommendation system, focusing on the data pipeline rather than a sophisticated recommendation algorithm.

The Goal: Process a stream of "user events" (e.g., article views) and generate a daily aggregate of popular articles.

Core Components & What You'll Learn:

Data Source: Simulated User Events (CSV/JSON Files)

What you'll learn: Understanding data schemas, simulating real-world data, working with common file formats.
Implementation: You'll manually create a few CSV or JSON files representing user interactions. Each row/object could contain:
user_id (string)
article_id (string)
timestamp (ISO 8601 string, e.g., 2025-05-30T10:30:00Z)
event_type (string, e.g., "view", "like")
Ingestion & Initial Transformation (Rust Application)

What you'll learn:
Rust for Data Parsing: Reading data from files, parsing CSV/JSON.
Basic Data Cleaning: Handling malformed records (e.g., missing fields).
Data Structures: Using Rust structs to represent your data.
Error Handling: Robustly handling potential issues during ingestion.
Performance (conceptual): How Rust's speed can be beneficial here, even if not fully realized in this simple project.
Implementation:
Write a Rust application that reads your simulated event files.
Parse each event into a Rust struct (e.g., UserEvent { user_id: String, article_id: String, timestamp: DateTime<Utc>, event_type: String }).
Filter out events that aren't "views" or have missing critical information.
Data Aggregation / Transformation (Rust Application)

What you'll learn:
Data Aggregation Logic: Counting occurrences, grouping data.
HashMaps/BTreeMaps: Using Rust's collection types for efficient aggregation.
Date/Time Manipulation: Grouping events by day.
Writing to Files: Outputting processed data.
Implementation:
Extend your Rust application from step 2.
For each "view" event, increment a counter for that article_id for the corresponding day.
The output of this step should be a collection of daily popular articles, perhaps looking like:
// For 2025-05-30
article_id_A, 150  // 150 views on this day
article_id_B, 120
// etc.
 Storage: Simple File Storage (CSV/Parquet)

What you'll learn:
Data Persistence: Saving your processed data.
File Formats for Analytics: Understanding why formats like Parquet are preferred over plain CSV for analytical workloads (even if you start with CSV for simplicity).
Rust CSV/Parquet Libraries: How to use external crates for writing these formats.
Implementation:
Your Rust application should write the daily aggregated results to new files.
Start with CSV for simplicity. Once comfortable, try writing to Parquet (using a crate like parquet or arrow-rs with Parquet features). This will introduce you to columnar storage concepts early. Store these in a processed_data/ directory.
Optional: Simple Orchestration (Bash Script or Python Script)

What you'll learn:
Automating Data Pipelines: How to chain together steps.
Basic Scripting: Running your Rust application programmatically.
Folder Structure: Organizing your data and code.
Implementation:
Write a simple run.sh or run.py script that does the following:
Ensures necessary directories exist.
Executes your Rust application.
(Optional) Moves or archives input files after processing.
```
