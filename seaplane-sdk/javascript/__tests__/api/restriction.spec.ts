import {afterEach, beforeAll, describe, expect, test} from '@jest/globals';

import { Configuration, Locks } from '../../src'
import MockAdapter from "axios-mock-adapter";

const axios = require('axios');

const mockIdentify = (mock: MockAdapter, configuration: Configuration) => {
  mock.onPost(`${configuration.identifyEndpoint}/identity/token`).reply(200, {token: "test_token"})
}

describe('Given Locks API', () => {

  test('get all pages ', async () => {
    
  });
  
});
