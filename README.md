# Uncle Good Advice

This repo contains library and binary executable to be used to build analitics tools on top of LLM's and other DeepLearning
models.

## Implementations and usages

### CLI Agent

BASIC-IMPLEMENTATION

CLI agent will build model with given prompt and allow to provide conversation with the model on given topic.

### Pull Agent

VERY BASIC IMPLEMENTATION

The Pull agent takes data from any source, such as Twitter or NewsDataIo, to analyze the data and store the results or send them to a receiver.

In its current state, it extracts relevant information such as keywords, coins, title, and text and passes it to the LLama model for sentiment recognition, then prints the results to the console.

The plan is to add storage (already implemented) or a receiver that can handle this data.

### Push Agent

NOT-IMPLEMENTED

Push agent can act as a simple REST API taking messages and responding with analitics, reporsts or assumptions.

### Examples:

- Trump Agent
  ![Trump](https://github.com/bartossh/uncle_good_advice/blob/master/artefacts/trump.png?raw=true)

- Sentiment Analitics Agent
  ![Sentiemnt example](https://github.com/bartossh/uncle_good_advice/blob/master/artefacts/sentiment.png?raw=true)

- Sentiment Pull Model - Pulls articles and calculates sentiment.
  ![Sentiment Pull Model](https://github.com/bartossh/uncle_good_advice/blob/master/artefacts/pull_model_sentiment.png?raw=true)
