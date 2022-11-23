import {afterEach, beforeAll, describe, expect, jest, test} from '@jest/globals';

import { Configuration, Metadata } from '../../src'
import MockAdapter from "axios-mock-adapter";

const axios = require('axios');

const mockIdentify = (mock: MockAdapter, configuration: Configuration) => {
  mock.onPost(`${configuration.identifyEndpoint}/identity/token`).reply(200, {token: "test_token"})
}

describe('Given Metadata API', () => {

  const config = new Configuration({ 
    apiKey: "test_apikey"
  })
  const metadata = new Metadata(config)
  let mockServer;

  beforeAll(() => {
    mockServer = new MockAdapter(axios)
    mockIdentify(mockServer, config)
  })

  afterEach(() => {
    mockServer.reset()
  })

  test('get page returns one element', async () => {  
    mockServer.onGet(`${config.coordinationEndpoint}/config`).reply(200, {
      kvs: [
        {
          key: "Zm9v",
          value: "YmFy",
        }
      ],
      next_key: null,
    })

      expect(await metadata.getPage()).toStrictEqual({
        keyValuePairs: [
          {
            key: "foo",
            value: "bar",
          }
        ],
        nextKey: null,
      })
  });
  
  test('get a key-value pair', async () => {
    mockServer.onGet(`${config.coordinationEndpoint}/config/base64:Zm9vL2Jhcg`).reply(200, {"key":"Zm9vL2Jhcg","value":"dmFsdWU"})

    expect(await metadata.get({key: "foo/bar"})).toStrictEqual({
      key: "foo/bar",
      value: "value"
    })
    
  })
  
  test('delete a key-value pair ', async () => {
    mockServer.onDelete(`${config.coordinationEndpoint}/config/base64:Zm9vL2Jhcg`).reply(200, "Ok")

    expect(await metadata.delete({key: "foo/bar"})).toBe(true)
  });

    
  test('set a key-value pair ', async () => {
    mockServer.onPut(`${config.coordinationEndpoint}/config/base64:YmFyL2Zvbw`).reply(200, "Ok")

    expect(await metadata.set({key: "bar/foo", value: "empty"})).toBe(true)
  });

  test('get page of directory ', async () => {
    mockServer.onGet(`${config.coordinationEndpoint}/config/base64:Zm9v/`).reply(200, {
      kvs: [
        {
          key: "Zm9v",
          value: "YmFy",
        }
      ],
      next_key: null,
    })

    expect(await metadata.getPage({directory: {key: "foo"}})).toStrictEqual({
      keyValuePairs: [
        {
          key: "foo",
          value: "bar",
        }
      ],
      nextKey: null,
    })
  });

  test('get next page ', async () => {
    mockServer.onGet(`${config.coordinationEndpoint}/config`, {params: { from: 'base64:Zm9v' }}).reply(200, {
      kvs: [
        {
          key: "Zm9v",
          value: "YmFy",
        }
      ],
      next_key: null,
    })

    expect(await metadata.getPage({nextKey: {key: "foo"}})).toStrictEqual({
      keyValuePairs: [
        {
          key: "foo",
          value: "bar",
        }
      ],
      nextKey: null,
    })
  });


  test('get all pages ', async () => {
    mockServer.onGet(`${config.coordinationEndpoint}/config`).reply(200, {
      kvs: [
        {
          key: "Zm9v",
          value: "YmFy",
        }
      ],
      next_key: null,
    })

    expect(await metadata.getAllPages()).toStrictEqual([{
      key: "foo",
      value: "bar",
    }]);
  });
  
});
