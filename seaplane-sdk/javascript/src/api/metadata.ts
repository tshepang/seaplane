import Configuration from '../configuration';
import Request, { headers } from './request';
import { Key, KeyValue, KeyValuePage, mapToKeyValue, mapToKeyValuePage } from '../model/metadata';

import { encode } from '../utils/base64';

const axios = require('axios'); // eslint-disable-line

export default class Metadata {
  url: string;
  request: Request;

  constructor(configuration: Configuration) {
    this.url = `${configuration.values().coordinationEndpoint}/config`;
    this.request = new Request(configuration.identify);
  }

  async set(keyValue: KeyValue): Promise<boolean> {
    const url = `${this.url}/base64:${encode(keyValue.key)}`;
    const data = encode(keyValue.value);

    const result = await this.request.send((token) =>
      axios.put(url, data, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      }),
    );

    return result === 'Ok';
  }

  async get(key: Key): Promise<KeyValue> {
    const url = `${this.url}/base64:${encode(key.key)}`;

    const result = await this.request.send((token) =>
      axios.get(url, {
        headers: headers(token),
      }),
    );

    return mapToKeyValue(result);
  }

  async delete(key: Key): Promise<boolean> {
    const url = `${this.url}/base64:${encode(key.key)}`;

    const result = await this.request.send((token) =>
      axios.delete(url, {
        headers: headers(token),
      }),
    );

    return result === 'Ok';
  }

  async getPage(options?: { directory?: Key; nextKey?: Key }): Promise<KeyValuePage> {
    let url = this.url;

    if (options?.directory) {
      url = `${this.url}/base64:${encode(options.directory.key)}/`;
    }

    let params = {};
    if (options?.nextKey) {
      params = {
        from: `base64:${encode(options.nextKey.key)}`,
      };
    }

    const result = await this.request.send((token) =>
      axios.get(url, {
        headers: headers(token),
        params: params,
      }),
    );

    return mapToKeyValuePage(result);
  }

  async getAllPages(options?: { directory?: Key; nextKey?: Key }): Promise<KeyValue[]> {
    const pages: KeyValue[] = [];
    let forNextKey = options?.nextKey;

    while (true) {
      const pageResult = await this.getPage({
        directory: options?.directory,
        nextKey: forNextKey,
      });

      const page: KeyValuePage = pageResult;
      pages.push(...page.keyValuePairs);

      if (page.nextKey) {
        forNextKey = page.nextKey;
      } else {
        return pages;
      }
    }
  }
}
