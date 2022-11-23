import {afterEach, beforeAll, describe, expect, jest, test} from '@jest/globals';

import { Configuration, Identify } from '../../src'
import MockAdapter from "axios-mock-adapter";

const axios = require('axios');

describe('Given Identify', () => {

  const config = new Configuration({ 
    apiKey: "test_apikey"
  })
  let mockServer;

  beforeAll(() => {
    mockServer = new MockAdapter(axios)  
  })

  afterEach(() => {
    mockServer.reset()
  })

  test('returns the token and save it locally', async() => {  
    const identify = new Identify(config)

    mockServer.onPost(`${config.identifyEndpoint}/identity/token`).reply(200, {token: "test_token"})

    await identify.getToken()

    expect(identify.accessToken).toBe("test_token")
  })

  test('autoRenew should be true', async() => {        
    const identify = new Identify(config)

    expect(identify.autoRenew).toBe(true)
  })

  test('autoRenew should be false when set the token', async() => {        
    const identify = new Identify(config)

    identify.setToken("this_is_a_token")

    expect(identify.autoRenew).toBe(false)
  })

  test('autoRenew should be false when set the token', async() => {        
    const identify = new Identify(config)

    identify.setToken("this_is_a_token")

    expect(identify.autoRenew).toBe(false)
  })

  test('accessToken should be the same as the set token', async() => {        
    const identify = new Identify(config)

    identify.setToken("this_is_a_token")

    expect(identify.accessToken).toBe("this_is_a_token")
  })

  test('accessToken should be the same as the set token', async() => {        
    const identify = new Identify(config)
    mockServer.onPost(`${config.identifyEndpoint}/identity/token`).reply(200, {token: "renewed_token"})    

    identify.setToken("this_is_a_token")    
    await identify.renewToken()

    expect(identify.accessToken).toBe("renewed_token")
  })

});