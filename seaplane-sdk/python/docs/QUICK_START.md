---
title: Python SDK Quickstart
excerpt: Get started quickly using Seaplane SDK for Python.
category: 62d96cf75848b100377f8554
slug: python-sdk-quickstart
---

# Introduction

Get started quickly using the Seaplane SDK for Python. This SDK makes it easy to integrate your Python application, library, or script with Seaplane features and services.

This guide details the steps needed to install, update, and use the Seaplane SDK for Python.

# Installation

## Install or Update Python

Before you install Seaplane Python SDK, install Python 3.7 or later.

## Install Seaplane Python SDK

⚠️ Install the Seaplane Python SDK. While this is not available yet, this is how it may look like.

```shell
pip install seaplane-python-sdk
```

## Configuration

Before using the Python SDK, you need to set up authentication credentials for your Seaplane account using the Flightdeck WebUI.

You can retrieve the API Key from Flightdeck WebUI and pass it to the Python SDK, using the `Configuration.set_api_key` method.

This is needed to set the API key before you start using the Seaplane Python SDK services and features.

## Python configuration

Seaplane Python SDK has the class `Configuration` which is used to configure the SDK.

There is a default configuration called, `config`:

```python
from seaplane import sea

sea.config.set_api_key("your-api-key-here")
```

# Usage

⚠️ This is not the final usage of the Python SDK, so please don't take this section too seriously.

To use the Seaplane Python SDK, you must first import it.

```python
from seaplane import sea
```

You can now create a formation easily using `FormationAPI`.

```python
from seaplane import sea
from seaplane.model import Formation, Flight

backend = Flight(
    name = "Backend Flight", 
    image = "registry.cplane.cloud/seaplane-demo/nginxdemos/hello:latest"
)

formation = Formation(
    name = "Seaplane-Services", 
    flights = [backend]
)

sea.formation.create(formation)
```

And that's it! You've got your Seaplane Python SDK ready to go.