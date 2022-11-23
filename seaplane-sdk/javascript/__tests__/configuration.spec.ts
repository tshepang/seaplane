import {afterEach, beforeAll, describe, expect, jest, test} from '@jest/globals';

import { Configuration } from '../src'

describe('Given Configuration', () => {

  test('returns the api when not initialized', () => {  
    let configuration = new Configuration({apiKey: "api_key"})

    expect(configuration.apiKey).toBe("api_key")
  })

  test('configuration token should be undefined by default', () => {  
    let configuration = new Configuration({apiKey: "api_key"})

    expect(configuration.accessToken).toBe(undefined)
  })

  test('autoRenew should be true by default', () => {  
    let configuration = new Configuration({apiKey: "api_key"})

    expect(configuration.autoRenew).toBe(true)
  })
  

});