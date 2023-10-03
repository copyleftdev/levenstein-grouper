
# Levenshtein-grouper
![Screenshot](Screenshot%202023-10-02%20222056.png)
## Overview

`Levenshtein-Grouper` is a powerful tool for computing the Levenshtein distances between lines of text in files within a specified directory. Designed with performance and scalability in mind, this tool is perfect for tasks that require text matching and comparison. It is especially useful in security contexts like duplicate identification, pattern recognition, and anomaly detection.

## Features

- **Multi-threaded Performance**: Leverages the Rayon library for parallel computing.
- **Progress Tracking**: Offers real-time progress updates with Indicatif.
- **JSON Export**: Capability to export the results into a dynamically named JSON file.
- **Fine-grained Control**: Set your own limits for Levenshtein distance calculations.
- **Color-coded Output**: Easy-to-read, color-coded terminal output.

## Installation

To install `Levenshtein-grouper`, clone the repository first:

```bash
git clone https://github.com/copyleftdev/levenshtein-grouper.git
```

Navigate into the project directory:

```bash
cd levenshtein-grouper
```

Compile the code:

```bash
cargo build --release
```

The compiled binary will be located under `target/release`.

## Usage

### Basic Usage

Run the following command to compute Levenshtein distances among text strings within files in a specific directory:

```bash
levenshtein-grouper --path /path/to/directory
```

### With Maximum Distance

To set a maximum limit for the Levenshtein distance in the calculations:

```bash
levenshtein-grouper --path /path/to/directory --distance 5
```

### JSON Export

To save the results as a dynamically named JSON file:

```bash
levenshtein-grouper--path /path/to/directory --json
```

## Use Cases

### Security

1. **Identifying Similar Code Blocks for Malware Analysis**: Detect segments of code that are almost identical across different malware families. This can help identify the techniques or algorithms commonly used by attackers.

2. **Phishing Email Detection**: Compare the contents of incoming emails with known phishing templates to flag suspicious emails.

3. **Password Strength Analysis**: Check if a new password is too similar to previously compromised passwords in a leaked database.

### Content Management

1. **Duplicate Content Detection**: Identify almost identical blocks of text across multiple documents or web pages to avoid SEO penalties for duplicate content.

2. **Text Similarity**: Measure the similarity between different versions of the same article or blog post. 

3. **Plagiarism Check**: Compare a document against a database of existing works to identify potential plagiarism.

### Natural Language Processing (NLP)

1. **Chatbots**: Improve the accuracy of chatbot responses by measuring the similarity between the user input and the pre-defined queries.

2. **Translation Memory Systems**: Find similar sentences or paragraphs in a corpus of previously translated text to assist human translators.

3. **Language Learning Apps**: Identify common mistakes or alternative answers in language learning exercises.

### Software Development

1. **Code Review**: Highlight lines of code that are nearly identical and may be candidates for refactoring into a function.

2. **Code Reusability**: Search for similar code blocks across projects to identify potential libraries or modules that could be created for reusability.

---

Feel free to expand or refine these points to better suit the features and capabilities of your `levenshtein-grouper` tool.

## Contributing

Contributions are welcome! Feel free to open a pull request.
