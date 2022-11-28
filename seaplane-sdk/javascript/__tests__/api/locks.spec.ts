import {afterEach, beforeAll, describe, expect, test} from '@jest/globals';

import { Configuration, Locks } from '../../src'
import MockAdapter from "axios-mock-adapter";

const axios = require('axios');

const mockIdentify = (mock: MockAdapter, configuration: Configuration) => {
  mock.onPost(`${configuration.identifyEndpoint}/identity/token`).reply(200, {token: "test_token"})
}

describe('Given Locks API', () => {

  const config = new Configuration({ 
    apiKey: "test_apikey"
  })
  const locks = new Locks(config)
  let mockServer;

  beforeAll(() => {
    mockServer = new MockAdapter(axios)
    mockIdentify(mockServer, config)
  })

  afterEach(() => {
    mockServer.reset()
  })

  test('get page returns one element', async () => {  
    mockServer.onGet(`${config.coordinationEndpoint}/locks`).reply(200, {
      "infos": [
          {
              "name": "bG9jay10ZXN0",
              "id": "BiqhSv0tuAk",
              "info": {"ttl": 1000, "client-id": "test", "ip": ""},
          }
      ],
      "next": null,
    })

      expect(await locks.getPage()).toStrictEqual({
        locks: [
          {
            id: "BiqhSv0tuAk",
            name: {
              name: "lock-test"
            },
            info: {ttl: 1000, clientId: "test", ip: ""},
          }
        ],
        nextLock: null,
      })
  });
  
  test('get a lock', async () => {
    mockServer.onGet(`${config.coordinationEndpoint}/locks/base64:Zm9vL2Jhcg`).reply(200, {
      "name": "Zm9vL2Jhcg",
      "id": "BiqhSv0tuAk",
      "info": {"ttl": 1000, "client-id": "test", "ip": ""},
  })

    expect(await locks.get({name: "foo/bar"})).toStrictEqual({
      name: {
        name: "foo/bar"
      },
      id: "BiqhSv0tuAk",
      info: {ttl: 1000, "clientId": "test", ip: ""},
  })
    
  })

  test('acquire a lock', async () => {    
    mockServer.onPost(`${config.coordinationEndpoint}/locks/base64:Zm9vL2Jhcg`).reply(200, {"id": "AOEHFRa4Ayg", "sequencer": 3})

    expect(await locks.acquire({name: "foo/bar"}, "client-id", 60)).toStrictEqual({
      id: "AOEHFRa4Ayg", 
      sequencer: 3
    })
  });

  test('release a lock', async () => {
    mockServer.onDelete(`${config.coordinationEndpoint}/locks/base64:Zm9vL2Jhcg`).reply(200, "OK")

    expect(await locks.release({name: "foo/bar"}, "id")).toBe(true)
  });


  test('get page of directory ', async () => {
    mockServer.onGet(`${config.coordinationEndpoint}/locks/base64:Zm9vL2Jhcg/`).reply(200, {
      infos: [
          {
              name: "Zm9vL2Jhcg",
              id: "BiqhSv0tuAk",
              info: {"ttl": 1000, "client-id": "test", "ip": ""},
          }
      ],
      next: null,
  })

    expect(await locks.getPage({directory: {name: "foo/bar"}})).toStrictEqual({
      locks: [
        {
          id: "BiqhSv0tuAk",
          name: {
            name: "foo/bar"
          },
          info: {
            clientId: "test",
            ip: "",
            ttl: 1000,
          }
        }
      ],
      nextLock: null,
    })
  });

  
  test('get next page ', async () => {
    mockServer.onGet(`${config.coordinationEndpoint}/locks`, {params: { from: 'base64:Zm9v' }}).reply(200, {
      infos: [
          {
              name: "Zm9vL2Jhcg",
              id: "BiqhSv0tuAk",
              info: {"ttl": 1000, "client-id": "test", "ip": ""},
          }
      ],
      next: null,
  })

    expect(await locks.getPage({fromLock: {name: "foo"}})).toStrictEqual({
      locks: [
        {
          id: "BiqhSv0tuAk",
          name: {
            name: "foo/bar"
          },
          info: {
            clientId: "test",
            ip: "",
            ttl: 1000,
          }
        }
      ],
      nextLock: null,
    })
  });

  test('get all pages ', async () => {
    mockServer.onGet(`${config.coordinationEndpoint}/locks`).reply(200, {
      infos: [
          {
              name: "Zm9vL2Jhcg",
              id: "BiqhSv0tuAk",
              info: {"ttl": 1000, "client-id": "test", "ip": ""},
          },
          {
            name: "Zm9v",
            id: "ASDF",
            info: {"ttl": 1000, "client-id": "test-id", "ip": ""},
        }
      ],
      next: null,
  })

    expect(await locks.getAllPages()).toStrictEqual([
      {
      id: "BiqhSv0tuAk",
      name: {
        name: "foo/bar"
      },
      info: {
        clientId: "test",
        ip: "",
        ttl: 1000,
      }
    },{
      id: "ASDF",
      name: {
        name: "foo"
      },
      info: {
        clientId: "test-id",
        ip: "",
        ttl: 1000,
      }
    }]);
  });
  
});
