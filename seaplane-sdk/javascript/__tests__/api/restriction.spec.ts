import {afterEach, beforeAll, describe, expect, jest, test} from '@jest/globals';
import { Configuration, Restrictions } from '../../src'
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

describe('Given Restrictions API', () => {

  const config = new Configuration({ 
    apiKey: "test_apikey"
  })
  const restrictions = new Restrictions(config)  

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
        "infos": [
            {
                "name": "bG9jay10ZXN0",
                "id": "BiqhSv0tuAk",
                "info": {"ttl": 1000, "client-id": "test", "ip": ""},
            }
        ],
        "next": null,
      })
    }))

    expect(await restrictions.getPage()).toStrictEqual({
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
  
  
});
