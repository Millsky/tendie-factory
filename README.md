# Tendie-Factory

## About

Tendie-Factory is a work in progress application that seeks to track the stocks mentioned in the `wallstreetbets` subreddit.

## Goals of the project

1. Create a portfolio of stocks based on the past 24 hours of data derived from `wallstreetbets`.
2. Determine if any of the stocks in the portfolio need to be rotated.
3. Automate the trading of these stocks via an API.
4. Profit!

## Current Behavior

Currently, the following algorithm is followed:

1. Pull the data from [WSB](https://www.reddit.com/r/wallstreetbets) top posts over the past day.
2. Use the [Named Entity Recognition Model](https://en.wikipedia.org/wiki/Named-entity_recognition) and Actively traded ticker symbols to parse out valid tickers.
3. For tickers that are mentioned, construct a portfolio of stock weights with `(Mentions / Total Mentions)`.
The program will output a JSON object with the tickers and respective weights, for example:

```json
{"GME": 0.8181818181818182, "BB": 0.18181818181818182}
```

## Citations

```bibtex
@inproceedings{becquin-2020-end,
    title = "End-to-end {NLP} Pipelines in Rust",
    author = "Becquin, Guillaume",
    booktitle = "Proceedings of Second Workshop for NLP Open Source Software (NLP-OSS)",
    year = "2020",
    publisher = "Association for Computational Linguistics",
    url = "https://www.aclweb.org/anthology/2020.nlposs-1.4",
    pages = "20--25",
}
```