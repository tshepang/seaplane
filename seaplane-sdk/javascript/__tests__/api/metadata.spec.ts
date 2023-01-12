import {afterEach, beforeAll, describe, expect, jest, test} from '@jest/globals';

import { Configuration, Metadata } from '../../src'
import seaFetch from '../../src/api/seaFetch';

jest.mock("../../src/api/seaFetch", () => jest.fn());

const mockIdentify = (configuration: Configuration) => {
  seaFetch.mockImplementation((token: string) => ({
    post: (url: string, body: string) => Promise.resolve({ 
      ok: () => true,
      json: () => Promise.resolve({token: "test_token"}) 
    })
  }))
}

const postTokenMock = {
  post: (url: string, body: string) => Promise.resolve({ 
    ok: () => true,
    json: () => Promise.resolve({token: "test_token"}) 
  })
}

const textBody = (body: Object) => Promise.resolve({ 
  ok: () => true,
  text: () => Promise.resolve(body) 
})

describe('Given Metadata API', () => {

  const config = new Configuration({ 
    apiKey: "test_apikey"
  })
  const metadata = new Metadata(config)  

  beforeAll(() => {
    mockIdentify(config)
  })

  afterEach(() => {
    seaFetch.mockClear()
  })

  test('get page returns one element', async () => {  
    seaFetch.mockImplementation((token: string) => ({
      ...postTokenMock,
      get: (url: string) => textBody({
        kvs: [
          {
            key: "Zm9v",
            value: "YmFy",
          }
        ],
        next_key: null,
        })
    }))    

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
    seaFetch.mockImplementation((token: string) => ({
      ...postTokenMock,
      get: (url: string) => textBody({"key":"Zm9vL2Jhcg","value":"dmFsdWU"})
    }))    

    expect(await metadata.get({key: "foo/bar"})).toStrictEqual({
      key: "foo/bar",
      value: "value"
    })
    
  })
  
  test('delete a key-value pair ', async () => {    
    seaFetch.mockImplementation((token: string) => ({
      ...postTokenMock,
      delete: (url: string) => textBody("Ok")
    }))  

    expect(await metadata.delete({key: "foo/bar"})).toBe(true)
  });

    
  test('set a key-value pair ', async () => {    
    seaFetch.mockImplementation((token: string) => ({
      ...postTokenMock,
      put:(url: string, body: string) => textBody("Ok")
    })) 

    expect(await metadata.set({key: "bar/foo", value: "empty"})).toBe(true)
  });

  test('get page of directory ', async () => {
    seaFetch.mockImplementation((token: string) => ({
      ...postTokenMock,
      get: (url: string) => textBody({
        kvs: [
          {
            key: "Zm9v",
            value: "YmFy",
          }
        ],
        next_key: null,
      })
    }))      

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
    seaFetch.mockImplementation((token: string) => ({
      ...postTokenMock,
      get: (url: string) => textBody({
        kvs: [
          {
            key: "Zm9v",
            value: "YmFy",
          }
        ],
        next_key: null,
      })
    }))     

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
    seaFetch.mockImplementation((token: string) => ({
      ...postTokenMock,
      get: (url: string) => textBody({
        kvs: [
          {
            key: "Zm9v",
            value: "YmFy",
          }
        ],
        next_key: null,
      })
    }))      

    expect(await metadata.getAllPages()).toStrictEqual([{
      key: "foo",
      value: "bar",
    }]);
  });
  
});
