---
title: Python SDK Quickstart
excerpt: Get started quickly using Seaplane SDK for Python.
category: 62d96cf75848b100377f8554
slug: python-sdk-quickstart
---

# Introduction

Get started quickly using Seaplane SDK for Python. It makes it easy to integrate your Python application, library, or script with Seaplane features and services.

This guide details the steps needed to install, update and use the Seaplane SDK for Python.

# Installation

## Install or update Python

Before install Seaplane Python SDK, install Python 3.7 or later.

## Install Python SDK

```shell
pip install seaplane-python-sdk
```

# Configuration

Before using the Python SDK, you need to set up authentication credentials for your Seaplane account using the Flightdeck WebUI.

You have to retrieve the API Key from Flightdeck WebUI and pass it to the Python SDK, using `Configuration.set_api_key` method.

It is needed to set the API key before start using the Seaplane Python SDK services and features.

## Python configuration

Seaplane Python SDK has the class `Configuration` which is used to configure the SDK.

There is a default configuration called, `config`:

```python
from seaplane import config

config.set_api_key("your-api-key-here")
```

# Usage

To use the Seaplane Python SDK, you must first import it.

```python
import seaplane
```

You can now create a formation easily using `FormationAPI`.

```python
from seaplane import FormationAPI
from seaplane.model import Formation, Flight

formation_api = FormationAPI()

backend = Flight(
    name = "Backend Flight", 
    image = "registry.cplane.cloud/seaplane-demo/nginxdemos/hello:latest"
)

formation = Formation(
    name = "Seaplane-Services", 
    flights = [backend]
)

formation_api.create(formation)
```
