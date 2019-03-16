## Goals

* Drift monitoring 
* Accuracy (and confusion matrix) monitoring 
* Extensibility (custom functions for monitoring)
* Stateless
* Able to be used in AWS Lambda, Containers, etc

## Open questions

* Choice of language (start with rust)
* Do we use Grafana or ELK or just use our own visualization? 
* If we use our own visualization, what libraries?

## Proposed architecture

* Thresholds set via yaml
* Simple accuracy (and other confusion matrix) metrics via yaml (timeframe to look back at, etc)
* Allow custom functions via CLI (which will generate the required code in the background with only the logic needed to be written)
* Custom functions will be pure functions

