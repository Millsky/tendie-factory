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

1. Pull the data from WSB top posts over the past day
2. Parse out the tickers that are valid
3. For tickers that are mentioned, construct a portfolio of stock weights with `(Mentions / Total Mentions)`.
The program will output a JSON object with the tickers and respective weights, for example:
```json
{"GME": 0.8181818181818182, "BB": 0.18181818181818182}
```